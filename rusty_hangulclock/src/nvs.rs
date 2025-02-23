use esp_idf_svc::nvs::*;
use log::info;

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASS: &str = env!("WIFI_PASS");

pub fn get_wifi_cred() -> anyhow::Result<(String, String)> {
    return Ok((WIFI_SSID.to_string(), WIFI_PASS.to_string()));
}
