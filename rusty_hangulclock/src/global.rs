// use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use lazy_static::lazy_static;
use std::sync::{
    // mpsc::{self, Receiver, Sender},
    Mutex,
};

lazy_static! {
    pub static ref TIME_SYNCED: Mutex<bool> = Mutex::new(false);
    pub static ref CMD_NET: Mutex<String> = Mutex::new(String::new());
    pub static ref LAST_DISP_TIME: Mutex<(u8, u8)> = Mutex::new((0, 0));
    pub static ref WIFI_IN_USE: Mutex<bool> = Mutex::new(false);
}
