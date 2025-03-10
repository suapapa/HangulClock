// use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use lazy_static::lazy_static;
use std::sync::{
    // mpsc::{self, Receiver, Sender},
    Mutex,
};

lazy_static! {
    pub static ref TIME_SYNCED: Mutex<bool> = Mutex::new(false);
    pub static ref CMD_NET: Mutex<String> = Mutex::new(String::new());
    pub static ref IN_MENU: Mutex<bool> = Mutex::new(false);
}
