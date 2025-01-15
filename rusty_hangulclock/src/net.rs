use crate::global;
use embassy_time::{Duration, Timer};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::sntp;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use esp_idf_svc::wifi::{WpsConfig, WpsFactoryInfo, WpsStatus, WpsType};
use log::{info, warn};

// const SSID: &str = env!("WIFI_SSID");
// const PASSWORD: &str = env!("WIFI_PASS");

const WPS_CONFIG: WpsConfig = WpsConfig {
    wps_type: WpsType::Pbc,
    factory_info: WpsFactoryInfo {
        manufacturer: "homin.dev",
        model_number: "hangulclock202501",
        model_name: "homin.dev IoT",
        device_name: "Rusty Hangul Clock",
    },
};

pub async fn connect_wps(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let mut wifi_configured = global::WIFI_CONFIGURED.lock().unwrap();

    wifi.start().await?;
    info!("Wifi started");

    match wifi.start_wps(&WPS_CONFIG).await? {
        WpsStatus::SuccessConnected => (),
        WpsStatus::SuccessMultipleAccessPoints(credentials) => {
            log::info!("received multiple credentials, connecting to first one:");
            for i in &credentials {
                log::info!(" - ssid: {}", i.ssid);
            }
            let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
                ssid: credentials[0].ssid.clone(),
                bssid: None,
                auth_method: AuthMethod::WPA2Personal,
                password: credentials[1].passphrase.clone(),
                channel: None,
                ..Default::default()
            });
            wifi.set_configuration(&wifi_configuration)?;
        }
        WpsStatus::Failure => anyhow::bail!("WPS failure"),
        WpsStatus::Timeout => anyhow::bail!("WPS timeout"),
        WpsStatus::Pin(_) => anyhow::bail!("WPS pin"),
        WpsStatus::PbcOverlap => anyhow::bail!("WPS PBC overlap"),
    }

    match wifi.get_configuration()? {
        Configuration::Client(config) => {
            info!("Successfully connected to {} using WPS", config.ssid)
        }
        _ => anyhow::bail!("Not in station mode"),
    };

    wifi.connect().await?;
    info!("Wifi connected");

    wifi.wait_netif_up().await?;
    info!("Wifi netif up");

    wifi.stop().await?;
    info!("Wifi stopped");

    *wifi_configured = true;

    Ok(())
}

pub async fn net_loop(
    wifi: &mut AsyncWifi<EspWifi<'static>>,
    mut debug_led: impl embedded_hal::digital::OutputPin,
) -> anyhow::Result<()> {
    // let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
    //     ssid: SSID.try_into().unwrap(),
    //     bssid: None,
    //     auth_method: AuthMethod::WPA2Personal,
    //     password: PASSWORD.try_into().unwrap(),
    //     channel: None,
    //     ..Default::default()
    // });

    // wifi.set_configuration(&wifi_configuration)?;
    debug_led.set_high().unwrap();
    let rx = global::CHAN_NET.1.lock().unwrap();
    for cmd in rx.iter() {
        match cmd.as_str() {
            "WPS" => {
                info!("Received WPS command");
                connect_wps(wifi).await?;
            }
            "NTP" => {
                info!("Received NTP command");
                let wifi_configured = { global::WIFI_CONFIGURED.lock().unwrap() };
                if !*wifi_configured {
                    info!("Connecting to wifi using WPS");
                    Timer::after(Duration::from_secs(3)).await;
                    continue;
                }

                {
                    info!("Resetting time_synced");
                    let mut time_synced = global::TIME_SYNCED.lock().unwrap();
                    *time_synced = false;
                }

                wifi.start().await?;
                info!("Wifi started");

                wifi.connect().await?;
                info!("Wifi connected");

                wifi.wait_netif_up().await?;
                info!("Wifi netif up");

                let sync_time_result = sync_time().await;
                if !sync_time_result {
                    warn!("Failed to sync time");
                }

                wifi.stop().await?;
                info!("Wifi stopped");
                {
                    info!("Setting time_synced");
                    let mut time_synced = global::TIME_SYNCED.lock().unwrap();
                    *time_synced = sync_time_result;
                }
                sync_time().await;
            }
            _ => {
                warn!("Unknown command: {}", cmd);
            }
        }

        debug_led.set_low().unwrap();
        // Timer::after(Duration::from_secs(60 * 60 * 24)).await;
        // Timer::after(Duration::from_secs(30)).await;
    }

    Ok(())
}

async fn sync_time() -> bool {
    let sntp = sntp::EspSntp::new_default().expect("Failed to create SNTP");
    let mut time_synced = false;
    let mut retry = 10;
    loop {
        if retry == 0 {
            break;
        }
        if sntp.get_sync_status() == sntp::SyncStatus::Completed {
            info!("SNTP synced");
            time_synced = true;
            break;
        }
        info!("Waiting for SNTP sync");
        // draw_text(&mut disp, "Waiting for\nSNTP sync...")?;
        // std::thread::sleep(time::Duration::from_secs(3));
        Timer::after(Duration::from_secs(3)).await;
        retry -= 1;
    }

    time_synced
}
