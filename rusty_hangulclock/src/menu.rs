// use embassy_time::{Duration, Timer};
use crate::global;
// use embassy_time::{Duration, Timer};
use esp_idf_svc::hal::i2c::*;
// use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
// use futures::FutureExt;
use log::info;
use sh1106::prelude::{GraphicsMode as Sh1106GM, I2cInterface};
use std::time;

pub async fn menu_loop(
    disp: &mut Sh1106GM<I2cInterface<I2cDriver<'_>>>,
    mut p_sel: impl embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
    // wifi: &mut AsyncWifi<EspWifi<'static>>,
) -> anyhow::Result<()> {
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X13, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        text::{Alignment, Text},
    };

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X13)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    let mut menu = 0;
    let menus = ["WPS", "Menu 2", "Menu 3"];
    let menu_max = menus.len();
    // let mut menu_selected = false;

    let mut sel_pressed: bool;
    let mut decide_pressed: bool;

    loop {
        sel_pressed = false;
        decide_pressed = false;

        p_sel.wait_for_low().await.unwrap();
        let ts_low = get_ts();
        p_sel.wait_for_high().await.unwrap();
        let ts_high = get_ts();
        if ts_high - ts_low < 500 {
            menu = (menu + 1) % menu_max;
            info!("sel press: {}", menu);
            // sel_pressed = true;
            decide_pressed = false;
        } else {
            info!("enter press: {}", menu);
            decide_pressed = true;
            // sel_pressed = false;
        }

        disp.clear();
        Text::with_alignment(
            menus[menu],
            Point::new(128 / 2, 64 / 2),
            text_style,
            Alignment::Center,
        )
        .draw(disp)?;
        disp.flush().unwrap();

        if decide_pressed {
            match menu {
                0 => {
                    info!("WPS selected");
                    // let tx = global::CHAN_NET.0.lock().unwrap().clone();
                    // tx.send("WPS".to_string()).unwrap();
                    let mut cmd_net = global::CMD_NET.lock().unwrap();
                    *cmd_net = "WPS".to_string();
                    info!("WPS cmd sent");
                }
                1 => {
                    info!("Menu 2 selected");
                    // Do something
                }
                2 => {
                    info!("Menu 3 selected");
                    // Do something
                }
                _ => {
                    info!("Unknown menu selected");
                }
            }
        }
    }
}

fn get_ts() -> u128 {
    let now = time::SystemTime::now();
    let timestamp = now.duration_since(time::UNIX_EPOCH).unwrap().as_millis();

    timestamp
}
