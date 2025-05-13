use crate::global;
use crate::nvs;
use crate::panel;
use embassy_time::{Duration, Timer};
use esp_idf_svc::hal::i2c::*;
// use esp_idf_svc::hal::task;
use log::info;
use sh1106::prelude::{GraphicsMode as Sh1106GM, I2cInterface};
use std::time;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuOption {
    Wps,
    Ntp,
    LedHue,
    LedSat,
    LedVal,
    UtcOffset,
    Exit,
}

impl MenuOption {
    fn as_str(&self) -> &'static str {
        match self {
            MenuOption::Wps => "WPS",
            MenuOption::Ntp => "NTP",
            MenuOption::LedHue => "LED HUE",
            MenuOption::LedSat => "LED SAT",
            MenuOption::LedVal => "LED VAL",
            MenuOption::UtcOffset => "UTC OFFSET",
            MenuOption::Exit => "EXIT",
        }
    }

    fn next(&self) -> Self {
        match self {
            MenuOption::Wps => MenuOption::Ntp,
            MenuOption::Ntp => MenuOption::LedHue,
            MenuOption::LedHue => MenuOption::LedSat,
            MenuOption::LedSat => MenuOption::LedVal,
            MenuOption::LedVal => MenuOption::UtcOffset,
            MenuOption::UtcOffset => MenuOption::Exit,
            MenuOption::Exit => MenuOption::Wps,
        }
    }

    fn prev(&self) -> Self {
        match self {
            MenuOption::Wps => MenuOption::Exit,
            MenuOption::Ntp => MenuOption::Wps,
            MenuOption::LedHue => MenuOption::Ntp,
            MenuOption::LedSat => MenuOption::LedHue,
            MenuOption::LedVal => MenuOption::LedSat,
            MenuOption::UtcOffset => MenuOption::LedVal,
            MenuOption::Exit => MenuOption::UtcOffset,
        }
    }

    fn all() -> [Self; 7] {
        [
            MenuOption::Wps,
            MenuOption::Ntp,
            MenuOption::LedHue,
            MenuOption::LedSat,
            MenuOption::LedVal,
            MenuOption::UtcOffset,
            MenuOption::Exit,
        ]
    }

    fn index(&self) -> usize {
        match self {
            MenuOption::Wps => 0,
            MenuOption::Ntp => 1,
            MenuOption::LedHue => 2,
            MenuOption::LedSat => 3,
            MenuOption::LedVal => 4,
            MenuOption::UtcOffset => 5,
            MenuOption::Exit => 6,
        }
    }
}

pub async fn menu_loop(
    disp: &mut Sh1106GM<I2cInterface<I2cDriver<'_>>>,
    mut p_sel: impl embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
) -> anyhow::Result<()> {
    info!("staring menu_loop()...");

    let mut current_menu = MenuOption::Wps;
    let menu_options = MenuOption::all();
    let menu_len = menu_options.len();

    let mut menu_enter_ts: u128 = get_ts();
    let mut sub_menu = false;

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
                        current_menu = MenuOption::Wps;
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
                let mut value = match current_menu {
                    MenuOption::LedHue => *global::LED_HUE.lock().unwrap() as i16,
                    MenuOption::LedSat => *global::LED_SAT.lock().unwrap() as i16,
                    MenuOption::LedVal => *global::LED_VAL.lock().unwrap() as i16,
                    MenuOption::UtcOffset => *global::UTC_OFFSET.lock().unwrap() as i16,
                    _ => 0,
                };

                draw_text(
                    disp,
                    &format!("= {} =\n{}\npress to\ndecide", current_menu.as_str(), value),
                )?;

                if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                    match *event {
                        global::RotaryEvent::Clockwise => {
                            match current_menu {
                                MenuOption::LedHue | MenuOption::LedSat | MenuOption::LedVal => {
                                    value += 5;
                                    if value > 255 {
                                        value = 255;
                                    }
                                }
                                MenuOption::UtcOffset => {
                                    value += 1;
                                    if value > 12 {
                                        value = 12;
                                    }
                                }
                                _ => {}
                            }
                            menu_enter_ts = get_ts();
                            *event = global::RotaryEvent::None;
                        }
                        global::RotaryEvent::CounterClockwise => {
                            match current_menu {
                                MenuOption::LedHue | MenuOption::LedSat | MenuOption::LedVal => {
                                    value -= 5;
                                    if value < 0 {
                                        value = 0;
                                    }
                                }
                                MenuOption::UtcOffset => {
                                    value -= 1;
                                    if value < -12 {
                                        value = -12;
                                    }
                                }
                                _ => {}
                            }
                            menu_enter_ts = get_ts();
                            *event = global::RotaryEvent::None;
                        }
                        _ => {}
                    }

                    match current_menu {
                        MenuOption::LedHue => *global::LED_HUE.lock().unwrap() = value as u8,
                        MenuOption::LedSat => *global::LED_SAT.lock().unwrap() = value as u8,
                        MenuOption::LedVal => *global::LED_VAL.lock().unwrap() = value as u8,
                        MenuOption::UtcOffset => *global::UTC_OFFSET.lock().unwrap() = value as i8,
                        _ => {}
                    }

                }


                if p_sel.is_low().unwrap() {
                    sub_menu = false;
                    Timer::after(Duration::from_millis(200)).await;
                    match current_menu {
                        MenuOption::LedHue | MenuOption::LedSat | MenuOption::LedVal => {
                            let hue = *global::LED_HUE.lock().unwrap();
                            let sat = *global::LED_SAT.lock().unwrap();
                            let val = *global::LED_VAL.lock().unwrap();
                            nvs::set_hsv(hue, sat, val).unwrap();
                        }
                        MenuOption::UtcOffset => {
                            let offset = *global::UTC_OFFSET.lock().unwrap();
                            nvs::set_utc_offset(offset as i32).unwrap();
                        }
                        _ => {}
                    }
                }
            } else {
                draw_text(
                    disp,
                    &format!(
                        "= MENU {}/{} =\n{}\npress to\ndecide",
                        current_menu.index() + 1,
                        menu_len,
                        current_menu.as_str()
                    ),
                )?;

                if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                    match *event {
                        global::RotaryEvent::Clockwise => {
                            current_menu = current_menu.next();
                            info!("Menu changed to: {:?}", current_menu);
                            menu_enter_ts = get_ts();
                            *event = global::RotaryEvent::None;
                        }
                        global::RotaryEvent::CounterClockwise => {
                            current_menu = current_menu.prev();
                            info!("Menu changed to: {:?}", current_menu);
                            menu_enter_ts = get_ts();
                            *event = global::RotaryEvent::None;
                        }
                        global::RotaryEvent::None => {
                            if p_sel.is_low().unwrap() {
                                info!("decide");
                                menu_enter_ts = get_ts();
                                match current_menu {
                                    MenuOption::Wps => {
                                        info!("WPS selected");
                                        match global::CMD_NET.try_lock() {
                                            Ok(mut cmd_net) => {
                                                draw_text(
                                                    disp,
                                                    &format!(
                                                        "MENU {}/{}\n**WPS**\nwait a moment",
                                                        current_menu.index() + 1,
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
                                                            current_menu.index() + 1,
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
                                    MenuOption::Ntp => {
                                        info!("NTP selected");
                                        match global::CMD_NET.try_lock() {
                                            Ok(mut cmd_net) => {
                                                draw_text(
                                                    disp,
                                                    &format!(
                                                        "MENU {}/{}\n**NTP**\nwait a moment",
                                                        current_menu.index() + 1,
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
                                                            current_menu.index() + 1,
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
                                    MenuOption::LedHue | MenuOption::LedSat | MenuOption::LedVal => {
                                        // LED color settings
                                        sub_menu = true;
                                        Timer::after(Duration::from_millis(200)).await;
                                    }
                                    MenuOption::UtcOffset => {
                                        // UTC OFFSET
                                        sub_menu = true;
                                        Timer::after(Duration::from_millis(200)).await;
                                    }
                                    MenuOption::Exit => {
                                        // EXIT
                                        info!("EXIT selected");
                                        draw_text(
                                            disp,
                                            &format!("MENU {}/{}\n**EXIT**", current_menu.index() + 1, menu_len,),
                                        )?;
                                        Timer::after(Duration::from_millis(1000)).await;
                                        *in_menu = false;
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
