use crate::global;
use embassy_time::{Duration, Ticker};
use log::info;
use rotary_encoder_hal::{Direction, Rotary};

pub async fn rotary_encoder_loop(
    menu_r1: impl embedded_hal::digital::InputPin,
    menu_r2: impl embedded_hal::digital::InputPin,
) -> anyhow::Result<()> {
    info!("Starting rotary_encoder_loop()...");

    let mut enc = Rotary::new(menu_r1, menu_r2);
    let mut ticker = Ticker::every(Duration::from_millis(10));
    let mut last_direction = Direction::None;
    let mut debounce_count = 0;
    const DEBOUNCE_THRESHOLD: u8 = 3; // Reduced threshold

    loop {
        match enc.update().unwrap() {
            Direction::Clockwise => {
                if last_direction != Direction::Clockwise {
                    debounce_count = 0;
                    last_direction = Direction::Clockwise;
                }
                debounce_count += 1;
                if debounce_count >= DEBOUNCE_THRESHOLD {
                    info!("Clockwise");
                    if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                        *event = global::RotaryEvent::Clockwise;
                    }
                    debounce_count = 0;
                }
            }
            Direction::CounterClockwise => {
                if last_direction != Direction::CounterClockwise {
                    debounce_count = 0;
                    last_direction = Direction::CounterClockwise;
                }
                debounce_count += 1;
                if debounce_count >= DEBOUNCE_THRESHOLD {
                    info!("CounterClockwise");
                    if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                        *event = global::RotaryEvent::CounterClockwise;
                    }
                    debounce_count = 0;
                }
            }
            _ => {
                // last_direction = Direction::None;
                // debounce_count = 0;
                // if let Ok(mut event) = global::ROTARY_EVENT.try_lock() {
                //     *event = global::RotaryEvent::None;
                // }
            }
        }
        ticker.next().await;
    }
} 