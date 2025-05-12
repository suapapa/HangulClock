use crate::global;
use embassy_time::{Duration, Timer};
use esp_idf_svc::hal::i2c::*;
// use esp_idf_svc::hal::task;
use log::info;
use sh1106::prelude::{GraphicsMode as Sh1106GM, I2cInterface};
use std::time;

pub async fn menu_loop(
    disp: &mut Sh1106GM<I2cInterface<I2cDriver<'_>>>,
    mut p_sel: impl embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
) -> anyhow::Result<()> {
    info!("staring menu_loop()...");

    let mut menu = 0;
    let menus = ["WPS", "NTP", "EXIT"];
    let menu_len = menus.len();

    let mut menu_enter_ts: u128 = get_ts();

    loop {
        Timer::after(Duration::from_millis(50)).await;
        let mut in_menu = global::IN_MENU.lock().unwrap();
        if !(*in_menu) {
            let h = *global::CUR_H.lock().unwrap();
            let m = *global::CUR_M.lock().unwrap();
            let time_str = format!("{:02}:{:02}", h, m);

            draw_text(
                disp,
                &format!("Rusty HangulClock\n{}\nrotate knob to\nenter menu", time_str),
            )?;
            if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                match *event {
                    global::RotaryEvent::Clockwise | global::RotaryEvent::CounterClockwise => {
                        *event = global::RotaryEvent::None;
                        info!("enter menu");
                        *in_menu = true;
                        menu = 0;
                        menu_enter_ts = get_ts();
                    }
                    _ => {}
                }
            }
        } else {
            let ts_now = get_ts();
            if (ts_now - menu_enter_ts) > 60 * 1000 {
                *in_menu = false;
                info!("exit menu");
                continue;
            }

            draw_text(
                disp,
                &format!(
                    "= MENU {}/{} =\n{}\npress to\ndecide",
                    menu + 1,
                    menu_len,
                    &(menus[menu])
                ),
            )?;

            if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                match *event {
                    global::RotaryEvent::Clockwise => {
                        menu = (menu + 1) % menu_len;
                        info!("Menu changed to: {}", menu);
                        *event = global::RotaryEvent::None;
                    }
                    global::RotaryEvent::CounterClockwise => {
                        menu = if menu == 0 { menu_len - 1 } else { menu - 1 };
                        info!("Menu changed to: {}", menu);
                        *event = global::RotaryEvent::None;
                    }
                    global::RotaryEvent::None => {
                        if p_sel.is_low().unwrap() {
                            info!("decide");
                            // *in_menu = false;
                            match menu {
                                0 => {
                                    info!("WPS selected");
                                    match global::CMD_NET.try_lock() {
                                        Ok(mut cmd_net) => {
                                            draw_text(
                                                disp,
                                                &format!(
                                                    "MENU {}/{}\n**WPS**\nwait a moment",
                                                    menu + 1,
                                                    menu_len,
                                                ),
                                            )?;
                                            *cmd_net = "WPS".to_string();
                                            info!("WPS cmd sent");
                                        }
                                        Err(_) => {
                                            info!("CMD_NET in use");
                                            continue;
                                        }
                                    }
                                    loop {
                                        Timer::after(Duration::from_millis(1000)).await;
                                        if let Ok(mut result) = global::RESULT_NET.try_lock() {
                                            if result.as_str() == "OK" || result.as_str() == "NG" {
                                                info!("WPS cmd completed");
                                                draw_text(
                                                    disp,
                                                    &format!(
                                                        "MENU {}/{}\nWPS\n**{}**",
                                                        menu + 1,
                                                        menu_len,
                                                        result.as_str(),
                                                    ),
                                                )?;
                                                Timer::after(Duration::from_millis(1000)).await;
                                                *in_menu = false;
                                                *result = "".to_string();
                                                break;
                                            }
                                        }
                                    }
                                }
                                1 => {
                                    info!("NTP selected");
                                    match global::CMD_NET.try_lock() {
                                        Ok(mut cmd_net) => {
                                            draw_text(
                                                disp,
                                                &format!(
                                                    "MENU {}/{}\n**NTP**\nwait a moment",
                                                    menu + 1,
                                                    menu_len,
                                                ),
                                            )?;
                                            *cmd_net = "NTP".to_string();
                                            info!("NTP cmd sent");
                                        }
                                        Err(_) => {
                                            info!("CMD_NET in use");
                                            continue;
                                        }
                                    }
                                    loop {
                                        Timer::after(Duration::from_millis(1000)).await;
                                        if let Ok(mut result) = global::RESULT_NET.try_lock() {
                                            if result.as_str() == "OK" || result.as_str() == "NG" {
                                                info!("NTP cmd completed");
                                                draw_text(
                                                    disp,
                                                    &format!(
                                                        "MENU {}/{}\nNTP\n**{}**",
                                                        menu + 1,
                                                        menu_len,
                                                        result.as_str(),
                                                    ),
                                                )?;
                                                Timer::after(Duration::from_millis(1000)).await;
                                                *in_menu = false;
                                                *result = "".to_string();
                                                break;
                                            }
                                        }
                                    }
                                }
                                2 => {
                                    info!("EXIT selected");
                                    draw_text(
                                        disp,
                                        &format!("MENU {}/{}\n**EXIT**", menu + 1, menu_len,),
                                    )?;
                                    Timer::after(Duration::from_millis(1000)).await;
                                    *in_menu = false;
                                }
                                _ => {
                                    info!("Unknown menu selected");
                                }
                            }
                        }
                    }
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

pub fn draw_text(disp: &mut Sh1106GM<I2cInterface<I2cDriver>>, text: &str) -> anyhow::Result<()> {
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

    disp.clear();
    Text::with_alignment(text, Point::new(128 / 2, 10), text_style, Alignment::Center)
        .draw(disp)?;
    disp.flush().unwrap();
    Ok(())
}
