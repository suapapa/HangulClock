use esp_idf_svc::nvs::*;
use log::info;

pub fn set_sleds(sled_type: &str) -> anyhow::Result<()> {
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let test_namespace = "sleds_ns";
    let mut nvs = match EspNvs::new(nvs_default_partition, test_namespace, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", test_namespace);
            nvs
        }
        Err(e) => return Err(anyhow::anyhow!("Could't get namespace {:?}", e)),
    };

    let tag = "sled_type";
    match nvs.set_str(tag, sled_type) {
        Ok(_) => info!("{:?} updated", tag),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", tag, e)),
    };

    Ok(())
}

pub fn get_sleds() -> anyhow::Result<String> {
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let test_namespace = "sleds_ns";
    let nvs = match EspNvs::new(nvs_default_partition, test_namespace, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", test_namespace);
            nvs
        }
        Err(e) => return Err(anyhow::anyhow!("Could't get namespace {:?}", e)),
    };

    // String values are limited in the IDF to 4000 bytes, but our buffer is shorter.
    const MAX_STR_LEN: usize = 100;
    let sled_tag = "sled_type";

    let sled_str_len: usize = nvs.str_len(sled_tag).map_or(0, |v| {
        info!("Got stored string length of {:?}", v);
        let vv = v.unwrap_or(0);
        if vv >= MAX_STR_LEN {
            info!("Too long, trimming");
            0
        } else {
            vv
        }
    });
    match sled_str_len == 0 {
        true => {
            info!("{:?} does not seem to exist", sled_tag);
            return Err(anyhow::anyhow!("Failed to get {:?}", sled_tag));
        }
        false => {
            let mut buffer: [u8; MAX_STR_LEN] = [0; MAX_STR_LEN];
            match nvs.get_str(sled_tag, &mut buffer).unwrap() {
                Some(v) => {
                    info!("{:?} = {:?}", sled_tag, v);
                    return Ok(v.to_string());
                }
                None => {
                    info!("We got nothing from {:?}", sled_tag);
                    return Err(anyhow::anyhow!("Failed to get {:?}", sled_tag));
                }
            };
        }
    };
}

pub fn set_wifi_cred(ssid: &str, pass: &str) -> anyhow::Result<()> {
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
        Ok(_) => info!("{:?} updated", ssid_tag),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", ssid_tag, e)),
    };
    match nvs.set_str(pass_tag, pass) {
        Ok(_) => info!("{:?} updated", pass_tag),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", pass_tag, e)),
    };

    Ok(())
}

pub fn get_wifi_cred() -> anyhow::Result<(String, String)> {
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
