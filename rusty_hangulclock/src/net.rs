use crate::global;
use crate::nvs;
use embassy_time::{Duration, Timer};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::hal::sys::esp_wifi_set_max_tx_power;
use esp_idf_svc::sntp;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use esp_idf_svc::wifi::{WpsConfig, WpsFactoryInfo, WpsStatus, WpsType};
use log::{info, warn};

pub async fn net_loop(
    wifi: &mut AsyncWifi<EspWifi<'static>>,
    // mut debug_led: impl embedded_hal::digital::OutputPin,
) -> anyhow::Result<()> {
    // debug_led.set_high().unwrap();

    loop {
        Timer::after(Duration::from_millis(100)).await;
        {
            let mut cmd_net = global::CMD_NET.lock().unwrap();

            match cmd_net.as_str() {
                "WPS" => {
                    info!("Received WPS command");
                    match connect_wps(wifi).await {
                        Ok(_) => {
                            info!("WPS cmd completed");
                            *cmd_net = "".to_string();
                            let mut result = global::RESULT_NET.lock().unwrap();
                            *result = "OK".to_string();
                        }
                        Err(e) => {
                            warn!("Failed to connect to wifi with wps: {:?}", e);
                            *cmd_net = "".to_string();
                            let mut result = global::RESULT_NET.lock().unwrap();
                            *result = "NG".to_string();
                        }
                    }
                }
                "NTP" => {
                    info!("Received NTP command");
                    match sync_time_with_wifi(wifi).await {
                        Ok(_) => {
                            info!("NTP cmd completed");
                            *cmd_net = "".to_string();
                            let mut result = global::RESULT_NET.lock().unwrap();
                            *result = "OK".to_string();
                        }
                        Err(e) => {
                            warn!("Failed to sync time: {:?}", e);
                            *cmd_net = "".to_string();
                            let mut result = global::RESULT_NET.lock().unwrap();
                            *result = "NG".to_string();
                        }
                    }
                }
                _ => {
                    // warn!("Unknown command: \"{}\"", cmd_net);
                }
            }

            // if cmd_net.as_str() != "" {
            //     // info!("Clearing command");
            //     // let mut cmd_net = global::CMD_NET.lock().unwrap();
            //     *cmd_net = "".to_string();
            // }
        }

        // debug_led.set_low().unwrap();
    }
}

const WPS_CONFIG: WpsConfig = WpsConfig {
    wps_type: WpsType::Pbc,
    factory_info: WpsFactoryInfo {
        manufacturer: "homin.dev",
        model_number: "hangulclock202501",
        model_name: "Rusty Hangul Clock",
        device_name: "Rusty Hangul Clock",
    },
};

pub async fn connect_wps(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: "homin_outside".try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: "homin_outside_pass".try_into().unwrap(),
        channel: None,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;

    wifi.start().await?;
    info!("Wifi started");

    unsafe { esp_wifi_set_max_tx_power(34) };

    info!("Starting WPS...");
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
            info!("Successfully connected to {} using WPS", config.ssid);
            nvs::set_wifi_cred(&config.ssid.clone(), &config.password.clone())?;
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

    return Ok(());
}

pub async fn sync_time_with_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<bool> {
    match nvs::get_wifi_cred() {
        Ok((ssid, pass)) => {
            let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
                ssid: ssid.as_str().try_into().unwrap(),
                bssid: None,
                auth_method: AuthMethod::WPA2Personal,
                password: pass.as_str().try_into().unwrap(),
                channel: None,
                ..Default::default()
            });

            wifi.set_configuration(&wifi_configuration)?;
        }
        Err(e) => {
            warn!("Failed to load wifi cred: {:?}", e);
            return Err(e);
        }
    }

    wifi.start().await?;
    info!("Wifi started");

    unsafe { esp_wifi_set_max_tx_power(34) };

    wifi.connect().await?;
    info!("Wifi connected");

    wifi.wait_netif_up().await?;
    info!("Wifi netif up");

    let sync_result = sync_time().await;
    if !sync_result {
        warn!("Failed to sync time");
    }

    wifi.stop().await?;
    info!("Wifi stopped");

    Ok(sync_result)
}

async fn sync_time() -> bool {
    let sntp = sntp::EspSntp::new_default().expect("Failed to create SNTP");
    let mut ret = false;
    let mut retry = 10;
    loop {
        if retry == 0 {
            break;
        }
        if sntp.get_sync_status() == sntp::SyncStatus::Completed {
            info!("SNTP synced");
            ret = true;
            break;
        }
        info!("Waiting for SNTP sync...");
        Timer::after(Duration::from_secs(3)).await;
        retry -= 1;
    }

    {
        info!("Setting time_synced");
        let mut time_synced = global::TIME_SYNCED.lock().unwrap();
        *time_synced = ret;
    }

    ret
}
