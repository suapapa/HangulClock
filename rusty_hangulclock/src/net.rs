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

    sync_time().await;
    info!("Time synced");

    wifi.stop().await?;
    info!("Wifi stopped");

    Ok(())
}

pub async fn net_loop(
    wifi: &mut AsyncWifi<EspWifi<'static>>,
    mut debug_led: impl embedded_hal::digital::OutputPin,
) -> anyhow::Result<()> {
    debug_led.set_high().unwrap();

    loop {
        let cmd_net: String;
        {
            cmd_net = global::CMD_NET.lock().unwrap().to_string();
        }

        match cmd_net.as_str() {
            "WPS" => {
                info!("Received WPS command");
                connect_wps(wifi).await?;
            }
            "NTP" => {
                info!("Received NTP command");
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

                sync_time().await;

                wifi.stop().await?;
                info!("Wifi stopped");
                {
                    info!("Setting time_synced");
                    let mut time_synced = global::TIME_SYNCED.lock().unwrap();
                    *time_synced = sync_time_result;
                }
            }
            _ => {
                // warn!("Unknown command: \"{}\"", cmd_net);
            }
        }

        if cmd_net.as_str() != "" {
            let mut cmd_net = { global::CMD_NET.lock().unwrap() };
            *cmd_net = "".to_string();
        }

        debug_led.set_low().unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
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
        info!("Waiting for SNTP sync...");
        Timer::after(Duration::from_secs(3)).await;
        retry -= 1;
    }

    time_synced
}
