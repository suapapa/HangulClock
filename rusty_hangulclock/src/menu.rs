use crate::global;
use crate::nvs;
use crate::panel;
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
    let menus = ["WPS", "NTP", "LED HUE", "LED SAT", "LED VAL", "EXIT"];
    let menu_len = menus.len();

    let mut menu_enter_ts: u128 = get_ts();
    let mut sub_menu = false;
    // let mut sub_menu_value = 0;

    loop {
        Timer::after(Duration::from_millis(50)).await;
        let mut in_menu = global::IN_MENU.lock().unwrap();
        if !(*in_menu) {
            let h = *global::CUR_H.lock().unwrap();
            let m = *global::CUR_M.lock().unwrap();
            let time_str = format!("{:02}:{:02}", h, m);

            draw_text(
                disp,
                &format!(
                    "Rusty HangulClock\n{}\nrotate knob to\nenter menu",
                    time_str
                ),
            )?;
            if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                match *event {
                    global::RotaryEvent::Clockwise | global::RotaryEvent::CounterClockwise => {
                        *event = global::RotaryEvent::None;
                        info!("enter menu");
                        *in_menu = true;
                        menu = 0;
                        sub_menu = false;
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

            if sub_menu {
                let mut value = match menu {
                    2 => *global::LED_HUE.lock().unwrap(),
                    3 => *global::LED_SAT.lock().unwrap(),
                    4 => *global::LED_VAL.lock().unwrap(),
                    _ => 0,
                };

                draw_text(
                    disp,
                    &format!("= {} =\n{}\npress to\ndecide", menus[menu], value),
                )?;

                if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                    match *event {
                        global::RotaryEvent::Clockwise => {
                            menu_enter_ts = get_ts();
                            value = value.saturating_add(1);
                            *event = global::RotaryEvent::None;
                        }
                        global::RotaryEvent::CounterClockwise => {
                            menu_enter_ts = get_ts();
                            value = value.saturating_sub(1);
                            *event = global::RotaryEvent::None;
                        }
                        _ => {}
                    }

                    match menu {
                        2 => *global::LED_HUE.lock().unwrap() = value,
                        3 => *global::LED_SAT.lock().unwrap() = value,
                        4 => *global::LED_VAL.lock().unwrap() = value,
                        _ => {}
                    }

                    if p_sel.is_low().unwrap() {
                        sub_menu = false;
                        Timer::after(Duration::from_millis(200)).await;
                        let hue = *global::LED_HUE.lock().unwrap();
                        let sat = *global::LED_SAT.lock().unwrap();
                        let val = *global::LED_VAL.lock().unwrap();
                        nvs::set_hsv(hue, sat, val).unwrap();
                    }
                }

                // sled.turn_on_all();
            } else {
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
                            menu_enter_ts = get_ts();
                            *event = global::RotaryEvent::None;
                        }
                        global::RotaryEvent::CounterClockwise => {
                            menu = if menu == 0 { menu_len - 1 } else { menu - 1 };
                            info!("Menu changed to: {}", menu);
                            menu_enter_ts = get_ts();
                            *event = global::RotaryEvent::None;
                        }
                        global::RotaryEvent::None => {
                            if p_sel.is_low().unwrap() {
                                info!("decide");
                                menu_enter_ts = get_ts();
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
                                                if result.as_str() == "OK"
                                                    || result.as_str() == "NG"
                                                {
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
                                                if result.as_str() == "OK"
                                                    || result.as_str() == "NG"
                                                {
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
                                    2 | 3 | 4 => {
                                        // LED color settings
                                        sub_menu = true;
                                        Timer::after(Duration::from_millis(200)).await;
                                    }
                                    5 => {
                                        // EXIT
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
}

fn get_ts() -> u128 {
    let now = time::SystemTime::now();
    let timestamp = now.duration_since(time::UNIX_EPOCH).unwrap().as_millis();

    timestamp
}

use std::sync::Mutex;
use once_cell::sync::Lazy;

static LAST_TEXT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn draw_text(disp: &mut Sh1106GM<I2cInterface<I2cDriver>>, text: &str) -> anyhow::Result<()> {
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X13, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        text::{Alignment, Text},
    };

    // last_text와 다를 때만 출력
    let mut last_text = LAST_TEXT.lock().unwrap();
    if *last_text == text {
        // 같으면 아무것도 하지 않음
        return Ok(());
    }
    // 다르면 업데이트
    *last_text = text.to_string();

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
