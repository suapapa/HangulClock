use crate::global;
use embassy_time::{Duration, Timer};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::nvs::*;
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
        Timer::after(Duration::from_secs(1)).await;
        {
            let mut cmd_net = global::CMD_NET.lock().unwrap();

            match cmd_net.as_str() {
                "WPS" => {
                    info!("Received WPS command");
                    match connect_wps(wifi).await {
                        Ok(_) => (),
                        Err(e) => {
                            warn!("Failed to connect to wifi with wps: {:?}", e);
                        }
                    }
                }
                "NTP" => {
                    info!("Received NTP command");
                    match sync_time_with_wifi(wifi).await {
                        Ok(_) => (),
                        Err(e) => {
                            warn!("Failed to sync time: {:?}", e);
                        }
                    }
                }
                _ => {
                    // warn!("Unknown command: \"{}\"", cmd_net);
                }
            }

            if cmd_net.as_str() != "" {
                // info!("Clearing command");
                // let mut cmd_net = global::CMD_NET.lock().unwrap();
                *cmd_net = "".to_string();
            }
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
            store_wifi_cred(&config.ssid.clone(), &config.password.clone())?;
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

async fn sync_time_with_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<bool> {
    match load_wifi_cred() {
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

fn store_wifi_cred(ssid: &str, pass: &str) -> anyhow::Result<()> {
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let test_namespace = "cred_ns";
    let mut nvs = match EspNvs::new(nvs_default_partition, test_namespace, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", test_namespace);
            nvs
        }
        Err(e) => return Err(anyhow::anyhow!("Could't get namespace {:?}", e)),
    };

    let ssid_tag = "ssid";
    let pass_tag = "pass";
    match nvs.set_str(ssid_tag, ssid) {
        Ok(_) => info!("{:?} updated", ssid),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", ssid_tag, e)),
    };
    match nvs.set_str(pass_tag, pass) {
        Ok(_) => info!("{:?} updated", pass),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", pass_tag, e)),
    };

    Ok(())
}

fn load_wifi_cred() -> anyhow::Result<(String, String)> {
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let test_namespace = "cred_ns";
    let nvs = match EspNvs::new(nvs_default_partition, test_namespace, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", test_namespace);
            nvs
        }
        Err(e) => return Err(anyhow::anyhow!("Could't get namespace {:?}", e)),
    };

    // String values are limited in the IDF to 4000 bytes, but our buffer is shorter.
    const MAX_STR_LEN: usize = 100;
    let ssid_tag = "ssid";
    let pass_tag = "pass";

    let ssid: String;
    let pass: String;

    let ssid_str_len: usize = nvs.str_len(ssid_tag).map_or(0, |v| {
        info!("Got stored string length of {:?}", v);
        let vv = v.unwrap_or(0);
        if vv >= MAX_STR_LEN {
            info!("Too long, trimming");
            0
        } else {
            vv
        }
    });
    match ssid_str_len == 0 {
        true => {
            info!("{:?} does not seem to exist", ssid_tag);
            return Err(anyhow::anyhow!("Failed to get {:?}", ssid_tag));
        }
        false => {
            let mut buffer: [u8; MAX_STR_LEN] = [0; MAX_STR_LEN];
            match nvs.get_str(ssid_tag, &mut buffer).unwrap() {
                Some(v) => {
                    info!("{:?} = {:?}", ssid_tag, v);
                    ssid = v.to_string();
                }
                None => {
                    info!("We got nothing from {:?}", ssid_tag);
                    return Err(anyhow::anyhow!("Failed to get {:?}", ssid_tag));
                }
            };
        }
    };
    let pass_str_len: usize = nvs.str_len(pass_tag).map_or(0, |v| {
        info!("Got stored string length of {:?}", v);
        let vv = v.unwrap_or(0);
        if vv >= MAX_STR_LEN {
            info!("Too long, trimming");
            0
        } else {
            vv
        }
    });
    match pass_str_len == 0 {
        true => {
            info!("{:?} does not seem to exist", pass_tag);
            return Err(anyhow::anyhow!("Failed to get {:?}", pass_tag));
        }
        false => {
            let mut buffer: [u8; MAX_STR_LEN] = [0; MAX_STR_LEN];
            match nvs.get_str(pass_tag, &mut buffer).unwrap() {
                Some(v) => {
                    info!("{:?} = {:?}", pass_tag, v);
                    pass = v.to_string();
                }
                None => {
                    info!("We got nothing from {:?}", pass_tag);
                    return Err(anyhow::anyhow!("Failed to get {:?}", pass_tag));
                }
            };
        }
    };

    Ok((ssid, pass))
}
