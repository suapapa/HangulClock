use chrono::prelude::*;
use embedded_hal::spi::MODE_3;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::spi::{
    config::Config as SpiConfig, config::DriverConfig as SpiDriverConfig, SpiBusDriver, SpiDriver,
};
use esp_idf_svc::hal::task::block_on;
use esp_idf_svc::sntp;
use esp_idf_svc::timer::EspTaskTimerService;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use log::info;
use sh1106::{
    prelude::{GraphicsMode as Sh1106GM, I2cInterface},
    Builder as Sh1106Builder,
};
use smart_leds::{gamma, hsv::hsv2rgb, hsv::Hsv, SmartLedsWrite, RGB8};
use std::time;

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    info!("Hello, world!");

    let p = Peripherals::take()?;

    let p_oled_sda = p.pins.gpio7;
    let p_oled_scl = p.pins.gpio9;
    let p_oled_res = p.pins.gpio3;
    let p_dotstar_sclk = p.pins.gpio34;
    let p_dotstar_mosi = p.pins.gpio35;
    let p_dotstar_spi = p.spi2;

    let sys_loop = EspSystemEventLoop::take()?;
    let timer_service = EspTaskTimerService::new()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut disp_res = PinDriver::output(p_oled_res)?;
    disp_res.set_low().unwrap();
    std::thread::sleep(time::Duration::from_millis(100));
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
    disp.flush().unwrap();
    draw_text(&mut disp, "Hello,\nworld!")?;

    let mut spi_driver = SpiDriver::new(
        p_dotstar_spi,
        p_dotstar_sclk,
        p_dotstar_mosi,
        AnyIOPin::none(),
        &SpiDriverConfig::new(),
    )?;
    let spi_config = SpiConfig::new().baudrate(3.MHz().into()).data_mode(MODE_3);
    let spi_bus = SpiBusDriver::new(&mut spi_driver, &spi_config)?;
    let mut dotstar = apa102_spi::Apa102::new(spi_bus);

    const LED_NUM: usize = 25;
    let mut dotstar_data = [RGB8::default(); LED_NUM];
    let mut hue: u16 = 0;
    for i in 0..LED_NUM {
        let color = hsv2rgb(Hsv {
            hue: hue as u8,
            sat: 255,
            val: 32,
        });
        dotstar_data[i] = color;
        hue = (hue + 256 / LED_NUM as u16) % 256;
    }
    dotstar.write(gamma(dotstar_data.iter().cloned())).unwrap();

    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(p.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
        timer_service,
    )?;
    block_on(connect_wifi(&mut wifi))?;

    let sntp = sntp::EspSntp::new_default().expect("Failed to create SNTP");
    loop {
        if sntp.get_sync_status() == sntp::SyncStatus::Completed {
            info!("SNTP synced");
            break;
        }
        info!("Waiting for SNTP sync");
        draw_text(&mut disp, "Waiting for\nSNTP sync...")?;
        std::thread::sleep(time::Duration::from_secs(3));
    }

    loop {
        let now = time::SystemTime::now();
        let timestamp = now.duration_since(time::UNIX_EPOCH).unwrap().as_millis();
        let datetime = Utc.timestamp_millis_opt(timestamp as i64).unwrap();
        let datetime_kst = datetime.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
        info!("Current datetime: {}", datetime_kst);
        draw_text(
            &mut disp,
            &datetime_kst.format("%Y-%m-%d\n%H:%M:%S").to_string(),
        )?;
        let h := datetime_kst.hour();
        let m := datetime_kst.minute();

        std::thread::sleep(time::Duration::from_secs(1));
    }
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

async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
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

    wifi.start().await?;
    info!("Wifi started");

    wifi.connect().await?;
    info!("Wifi connected");

    wifi.wait_netif_up().await?;
    info!("Wifi netif up");

    Ok(())
}
