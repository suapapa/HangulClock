use esp_idf_svc::nvs::*;
use log::info;

pub fn set_wifi_cred(ssid: &str, pass: &str) -> anyhow::Result<()> {
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let ns = "cred_ns";
    let mut nvs = match EspNvs::new(nvs_default_partition, ns, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", ns);
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

    let ns = "cred_ns";
    let nvs = match EspNvs::new(nvs_default_partition, ns, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", ns);
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

pub fn set_hsv(hue: u8, sat: u8, val: u8) -> anyhow::Result<()> {
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let ns = "hsv_ns";
    let nvs = match EspNvs::new(nvs_default_partition, ns, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", ns);
            nvs
        }
        Err(e) => return Err(anyhow::anyhow!("Could't get namespace {:?}", e)),
    };

    let hue_tag = "hue";
    let sat_tag = "sat";
    let val_tag = "val";
    
    match nvs.set_u8(hue_tag, hue) {
        Ok(_) => info!("{:?} updated", hue_tag),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", hue_tag, e)),
    };
    match nvs.set_u8(sat_tag, sat) {
        Ok(_) => info!("{:?} updated", sat_tag),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", sat_tag, e)),
    };
    match nvs.set_u8(val_tag, val) {
        Ok(_) => info!("{:?} updated", val_tag),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", val_tag, e)),
    };
    Ok(())
}

pub fn get_hsv() -> anyhow::Result<(u8, u8, u8)> {  
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let ns = "hsv_ns";
    let nvs = match EspNvs::new(nvs_default_partition, ns, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", ns);
            nvs
        }
        Err(e) => return Err(anyhow::anyhow!("Could't get namespace {:?}", e)),
    };

    let hue_tag = "hue";
    let sat_tag = "sat";
    let val_tag = "val";

    let hue = nvs.get_u8(hue_tag)
        .map(|v| v.unwrap_or(0))
        .unwrap_or(0);
    let sat = nvs.get_u8(sat_tag)
        .map(|v| v.unwrap_or(255))
        .unwrap_or(255);
    let val = nvs.get_u8(val_tag)
        .map(|v| v.unwrap_or(255))
        .unwrap_or(255);

    Ok((hue, sat, val)) 
}

pub fn set_utc_offset(offset: i32) -> anyhow::Result<()> {
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let ns = "utc_offset_ns";
    let nvs = match EspNvs::new(nvs_default_partition, ns, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", ns);
            nvs
        }
        Err(e) => return Err(anyhow::anyhow!("Could't get namespace {:?}", e)),
    };

    let offset_tag = "offset";
    match nvs.set_i32(offset_tag, offset) {
        Ok(_) => info!("{:?} updated", offset_tag),
        Err(e) => return Err(anyhow::anyhow!("Failed to set {:?}: {:?}", offset_tag, e)),
    };

    Ok(())
}

pub fn get_utc_offset() -> anyhow::Result<i32> {
    let nvs_default_partition: EspNvsPartition<NvsCustom> =
        EspCustomNvsPartition::take("user_nvs")?;

    let ns = "utc_offset_ns";
    let nvs = match EspNvs::new(nvs_default_partition, ns, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", ns);
            nvs
        }
        Err(e) => return Err(anyhow::anyhow!("Could't get namespace {:?}", e)),
    };

    let offset_tag = "offset";
    let offset = nvs.get_i32(offset_tag)
        .map(|v| v.unwrap_or(9)) // default offset is 9, Asia/Seoul
        .unwrap_or(0);

    Ok(offset)
}
