use crate::global;
use embassy_time::{Duration, Timer};
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::task;
use log::info;
use sh1106::prelude::{GraphicsMode as Sh1106GM, I2cInterface};
use std::time;

pub async fn menu_loop(
    disp: &mut Sh1106GM<I2cInterface<I2cDriver<'_>>>,
    mut p_sel: impl embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
) -> anyhow::Result<()> {
    loop {
        // task::yield_now().await;
        p_sel.wait_for_low().await.unwrap();
        info!("WPS selected");
        draw_text(disp, "* WPS *")?;
        {
            let mut cmd_net = global::CMD_NET.lock().unwrap();
            *cmd_net = "WPS".to_string();
        }
        info!("WPS cmd sent");
        // task::yield_now().await;
        Timer::after(Duration::from_secs(5)).await;
    }
}
/*
pub async fn menu_loop(
    disp: &mut Sh1106GM<I2cInterface<I2cDriver<'_>>>,
    mut p_sel: impl embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
) -> anyhow::Result<()> {
    let mut in_menu = false;

    let mut menu = 0;
    let menus = ["WPS", "NTP", "EXIT"];
    let menu_max = menus.len();

    let mut decide_pressed: bool;

    let mut last_disp_time: (u8, u8) = (0, 0);
    loop {
        if !in_menu {
            if p_sel.is_low().unwrap() {
                info!("enter menu");
                in_menu = true;
                menu = 0;
                continue;
            }

            {
                let disp_time = global::LAST_DISP_TIME.lock().unwrap();
                if disp_time.0 != last_disp_time.0 || disp_time.1 != last_disp_time.1 {
                    info!("disp_time: {:02}:{:02}", disp_time.0, disp_time.1);
                    draw_text(
                        disp,
                        &format!("Rusty HangulClock\n{:02}:{:02}", disp_time.0, disp_time.1),
                    )?;
                    last_disp_time = *disp_time;
                }
            }
            Timer::after(Duration::from_secs(5)).await;
            continue;
        }

        draw_text(disp, menus[menu])?;

        p_sel.wait_for_low().await.unwrap();
        let ts_low = get_ts();
        p_sel.wait_for_high().await.unwrap();
        let ts_high = get_ts();
        if ts_high - ts_low < 500 {
            menu = (menu + 1) % menu_max;
            info!("sel press: {}", menu);
            decide_pressed = false;
        } else {
            info!("enter press: {}", menu);
            decide_pressed = true;
        }

        draw_text(disp, menus[menu])?;

        if decide_pressed {
            match menu {
                0 => {
                    info!("WPS selected");
                    draw_text(disp, "* WPS *")?;
                    let mut cmd_net = global::CMD_NET.lock().unwrap();
                    *cmd_net = "WPS".to_string();
                    info!("WPS cmd sent");
                }
                1 => {
                    info!("NTP selected");
                    draw_text(disp, "* NPT *")?;
                    let mut cmd_net = global::CMD_NET.lock().unwrap();
                    *cmd_net = "NTP".to_string();
                    info!("NTP cmd sent");
                }
                2 => {
                    info!("EXIT selected");
                    draw_text(disp, "* EXIT *")?;
                    in_menu = false;
                    // Do something
                }
                _ => {
                    info!("Unknown menu selected");
                }
            }
        }
    }
}
 */

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
    Text::with_alignment(
        text,
        Point::new(128 / 2, 64 / 2),
        text_style,
        Alignment::Center,
    )
    .draw(disp)?;
    disp.flush().unwrap();
    Ok(())
}
