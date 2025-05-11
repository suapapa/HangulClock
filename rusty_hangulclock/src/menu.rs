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
    let menu_max = menus.len();

    let mut decide_pressed: bool;

    draw_text(disp, &format!("Rusty HangulClock\npress to enter menu"))?;
    loop {
        Timer::after(Duration::from_millis(300)).await;
        {
            let mut in_menu = global::IN_MENU.lock().unwrap();
            if !(*in_menu) {
                draw_text(disp, &format!("Rusty HangulClock\npress to enter menu"))?;
                if p_sel.is_low().unwrap() {
                    info!("enter menu");
                    *in_menu = true;
                    menu = 0;
                }
                continue;
            }
        }

        draw_text(
            disp,
            &format!("= MENU =\n{}\ns:next\nl:decide", &(menus[menu])),
        )?;

        // Check rotary encoder events
        if let Ok(event) = global::ROTARY_EVENT.try_lock() {
            match *event {
                global::RotaryEvent::Clockwise => {
                    menu = (menu + 1) % menu_max;
                    info!("Menu changed to: {}", menu);
                }
                global::RotaryEvent::CounterClockwise => {
                    menu = if menu == 0 { menu_max - 1 } else { menu - 1 };
                    info!("Menu changed to: {}", menu);
                }
                global::RotaryEvent::None => {}
            }
        }

        p_sel.wait_for_low().await.unwrap();
        let ts_low = get_ts();
        p_sel.wait_for_high().await.unwrap();
        let ts_high = get_ts();
        if ts_high - ts_low < 500 {
            info!("sel press: {}", menu);
            decide_pressed = false;
        } else {
            info!("enter press: {}", menu);
            decide_pressed = true;
        }

        if !decide_pressed {
            // change menu
            menu = (menu + 1) % menu_max;
        }

        if decide_pressed {
            let mut in_menu = global::IN_MENU.lock().unwrap();
            match menu {
                0 => {
                    info!("WPS selected");
                    match global::CMD_NET.try_lock() {
                        Ok(mut cmd_net) => {
                            draw_text(disp, "* WPS *")?;
                            *cmd_net = "WPS".to_string();
                            info!("WPS cmd sent");
                        }
                        Err(_) => {
                            info!("CMD_NET in use");
                        }
                    }
                }
                1 => {
                    info!("NTP selected");
                    match global::CMD_NET.try_lock() {
                        Ok(mut cmd_net) => {
                            draw_text(disp, "* NPT *")?;
                            *cmd_net = "NTP".to_string();
                            info!("NTP cmd sent");
                        }
                        Err(_) => {
                            info!("CMD_NET in use");
                        }
                    }
                }
                2 => {
                    info!("EXIT selected");
                    draw_text(disp, "* EXIT *")?;
                    *in_menu = false;
                }
                _ => {
                    info!("Unknown menu selected");
                }
            }

            loop {
                Timer::after(Duration::from_secs(30)).await;
                match global::CMD_NET.try_lock() {
                    Ok(cmd_net) => {
                        if cmd_net.as_str() == "" {
                            *in_menu = false;
                            break;
                        }
                    }
                    Err(_) => {
                        info!("CMD_NET in use");
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
