// mod panel_apa102;
mod panel_ws2812;

use chrono::prelude::*;
use embassy_time::{Delay, Duration, Timer};
use embedded_hal::spi::MODE_3;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::spi::{
    config::Config as SpiConfig, config::DriverConfig as SpiDriverConfig, SpiBusDriver, SpiDriver,
};
use esp_idf_svc::hal::task;
use esp_idf_svc::sntp;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use log::{debug, info, warn};
use sh1106::{
    prelude::{GraphicsMode as Sh1106GM, I2cInterface},
    Builder as Sh1106Builder,
};
use ws2812_spi::{Ws2812, MODE as Ws2812_MODE};
// use smart_leds::{gamma, hsv::hsv2rgb, hsv::Hsv, SmartLedsWrite, RGB8};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time;

lazy_static! {
    static ref TIME_SYNCED: Mutex<bool> = Mutex::new(false);
}

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    info!("Hello, world!");

    let p = Peripherals::take()?;

    let p_oled_sda = p.pins.gpio8;
    let p_oled_scl = p.pins.gpio9;
    let p_oled_res = p.pins.gpio10;
    let p_sled_sclk = p.pins.gpio4;
    let p_sled_mosi = p.pins.gpio6;
    let p_sled_spi = p.spi2;

    let sys_loop = EspSystemEventLoop::take()?;
    let timer_service = EspTaskTimerService::new()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut disp_res = PinDriver::output(p_oled_res)?;
    disp_res.set_low().unwrap();
    std::thread::sleep(time::Duration::from_millis(100));
    // Timer::after(Duration::from_millis(100)).await;
    disp_res.set_high().unwrap();

    let i2c_config = I2cConfig::new().baudrate(50.kHz().into());
    let i2c = I2cDriver::new(
        p.i2c0,
        p_oled_sda, // SDA
        p_oled_scl, // SCL
        &i2c_config,
    )?;
    let mut disp: Sh1106GM<_> = Sh1106Builder::new().connect_i2c(i2c).into();
    disp.init().unwrap();
    // disp.set_rotation(sh1106::prelude::DisplayRotation::Rotate180)
    //     .unwrap();
    disp.flush().unwrap();
    draw_text(&mut disp, "Hello,\nworld!")?;

    let mut spi_driver = SpiDriver::new(
        p_sled_spi,
        p_sled_sclk,
        p_sled_mosi,
        AnyIOPin::none(),
        &SpiDriverConfig::new(),
    )?;
    let spi_config = SpiConfig::new()
        .baudrate(3.MHz().into())
        .data_mode(Ws2812_MODE);
    let spi_bus = SpiBusDriver::new(&mut spi_driver, &spi_config)?;
    // let mut sleds = apa102_spi::Apa102::new(spi_bus);

    // let mut sled_buf: [u8; 12 * 25 + 140] = [0; 12 * 25 + 140];
    // let mut sleds = Ws2812::new(spi_bus, &mut sled_buf);
    let mut sleds = Ws2812::new(spi_bus);

    panel_ws2812::welcome(&mut sleds);

    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(p.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
        timer_service,
    )?;
    // task::block_on(time_sync_loop(&mut wifi))?;
    let time_sync_task = time_sync_loop(&mut wifi);
    let show_time_task = show_time_loop(&mut sleds);

    task::block_on(async {
        match futures::try_join!(time_sync_task, show_time_task) {
            Ok(_) => info!("All tasks completed"),
            Err(e) => info!("Error in task: {:?}", e),
        }
    });

    // let sntp = sntp::EspSntp::new_default().expect("Failed to create SNTP");
    // loop {
    //     if sntp.get_sync_status() == sntp::SyncStatus::Completed {
    //         info!("SNTP synced");
    //         break;
    //     }
    //     info!("Waiting for SNTP sync");
    //     draw_text(&mut disp, "Waiting for\nSNTP sync...")?;
    //     std::thread::sleep(time::Duration::from_secs(3));
    // }

    // let mut last_h = 0;
    // let mut last_m = 0;
    // loop {
    //     let now = time::SystemTime::now();
    //     let timestamp = now.duration_since(time::UNIX_EPOCH).unwrap().as_millis();
    //     let datetime = Utc.timestamp_millis_opt(timestamp as i64).unwrap();
    //     let datetime_kst = datetime.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
    //     info!("Current datetime: {}", datetime_kst);
    //     draw_text(
    //         &mut disp,
    //         &datetime_kst.format("%Y-%m-%d\n%H:%M:%S").to_string(),
    //     )?;
    //     let h: u8 = datetime_kst.hour() as u8;
    //     let m: u8 = datetime_kst.minute() as u8;

    //     if last_h != h || last_m != m {
    //         last_h = h;
    //         last_m = m;
    //         info!("Time updated");
    //     }
    //     panel_ws2812::show_time(&mut sleds, h, m);
    //     std::thread::sleep(time::Duration::from_secs(1));
    // }
    Ok(())
}

async fn show_time_loop<SPI>(sleds: &mut Ws2812<SPI>) -> anyhow::Result<()>
where
    SPI: embedded_hal::spi::SpiBus,
{
    let mut last_h = 0;
    let mut last_m = 0;
    loop {
        let mut skip_loop = false;
        {
            let time_synced = TIME_SYNCED.lock().unwrap();
            if !*time_synced {
                warn!("Time not synced yet");
                skip_loop = true;
            }
        }

        if skip_loop {
            // task::yield_now().await;
            Timer::after(Duration::from_secs(10)).await; //std::thread::sleep(time::Duration::from_secs(10));
            continue;
        }

        let now = time::SystemTime::now();
        let timestamp = now.duration_since(time::UNIX_EPOCH).unwrap().as_millis();
        let datetime = Utc.timestamp_millis_opt(timestamp as i64).unwrap();
        let datetime_kst = datetime.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
        info!("Current datetime: {}", datetime_kst);
        // draw_text(
        //     &mut disp,
        //     &datetime_kst.format("%Y-%m-%d\n%H:%M:%S").to_string(),
        // )?;
        let h: u8 = datetime_kst.hour() as u8;
        let m: u8 = datetime_kst.minute() as u8;

        if last_h != h || last_m != m {
            last_h = h;
            last_m = m;
            info!("Time updated");
            panel_ws2812::show_time(sleds, h, m);
        }
        // std::thread::sleep(time::Duration::from_secs(1));
        Timer::after(Duration::from_secs(1)).await;
    }
    // Ok(())
}

fn draw_text(disp: &mut Sh1106GM<I2cInterface<I2cDriver>>, text: &str) -> anyhow::Result<()> {
    use embedded_graphics::{
        mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        text::{Alignment, Text},
    };

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    disp.clear();
    Text::with_alignment(
        text,
        Point::new(128 / 2, 64 / 2),
        text_style,
        Alignment::Center,
    )
    .draw(disp)?;
    disp.flush().unwrap();
    Ok(())
}

async fn time_sync_loop(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    loop {
        {
            info!("Resetting time_synced");
            let mut time_synced = TIME_SYNCED.lock().unwrap();
            *time_synced = false;
        }

        wifi.start().await?;
        info!("Wifi started");

        wifi.connect().await?;
        info!("Wifi connected");

        wifi.wait_netif_up().await?;
        info!("Wifi netif up");

        let sntp = sntp::EspSntp::new_default().expect("Failed to create SNTP");
        loop {
            if sntp.get_sync_status() == sntp::SyncStatus::Completed {
                info!("SNTP synced");
                break;
            }
            info!("Waiting for SNTP sync");
            // draw_text(&mut disp, "Waiting for\nSNTP sync...")?;
            // std::thread::sleep(time::Duration::from_secs(3));
            Timer::after(Duration::from_secs(3)).await;
        }

        wifi.stop().await?;
        info!("Wifi stopped");
        {
            info!("Setting time_synced");
            let mut time_synced = TIME_SYNCED.lock().unwrap();
            *time_synced = true;
        }
        // task::yield_now().await;
        // std::thread::sleep(time::Duration::from_secs(60 * 60 * 24)); // 24 hours
        Timer::after(Duration::from_secs(60 * 60 * 24)).await;
    }
}
