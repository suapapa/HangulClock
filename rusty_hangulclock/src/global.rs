// use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use lazy_static::lazy_static;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

lazy_static! {
    pub static ref WIFI_CONFIGURED: Mutex<bool> = Mutex::new(false);
    pub static ref TIME_SYNCED: Mutex<bool> = Mutex::new(false);
    // pub static ref WIFI: Mutex<Arc<AsyncWifi<EspWifi<'static>>>> = Mutex::new(None);

    // mpsc 채널을 Mutex로 감싸서 전역으로 선언
    pub static ref CHAN_NET: (Mutex<Sender<String>>, Mutex<Receiver<String>>) = {
        let (tx, rx) = mpsc::channel();
        (Mutex::new(tx), Mutex::new(rx))
    };
}
