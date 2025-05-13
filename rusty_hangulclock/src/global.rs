// use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use lazy_static::lazy_static;
use std::sync::{
    // mpsc::{self, Receiver, Sender},
    Mutex,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotaryEvent {
    None,
    Clockwise,
    CounterClockwise,
}

lazy_static! {
    pub static ref TIME_SYNCED: Mutex<bool> = Mutex::new(false);
    pub static ref CMD_NET: Mutex<String> = Mutex::new(String::new());
    pub static ref RESULT_NET: Mutex<String> = Mutex::new(String::new());
    pub static ref IN_MENU: Mutex<bool> = Mutex::new(false);
    pub static ref ROTARY_EVENT: Mutex<RotaryEvent> = Mutex::new(RotaryEvent::None);
    pub static ref CUR_H: Mutex<u8> = Mutex::new(0);
    pub static ref CUR_M: Mutex<u8> = Mutex::new(0);
    pub static ref LED_HUE: Mutex<u8> = Mutex::new(0);
    pub static ref LED_SAT: Mutex<u8> = Mutex::new(255);
    pub static ref LED_VAL: Mutex<u8> = Mutex::new(255);
    pub static ref UTC_OFFSET: Mutex<i8> = Mutex::new(9); // Default to KST (UTC+9)
}
