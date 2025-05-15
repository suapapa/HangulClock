#[cfg(feature = "use_dotstar")]
use apa102_spi::Apa102;
#[cfg(not(feature = "use_dotstar"))]
use ws2812_spi::Ws2812;

use embedded_hal::spi::SpiBus;
use log::info;
use smart_leds::{gamma, hsv::hsv2rgb, hsv::Hsv, SmartLedsWrite, RGB8};
use std::sync::{Arc, Mutex};

use crate::global;
use crate::nvs;

const LED_NUM: usize = 25;
// const DEFAULT_BRIGHTNESS: u8 = 100;

#[cfg(feature = "use_dotstar")]
pub struct Sleds<SPI> {
    sleds: Arc<Mutex<Apa102<SPI>>>,
}

#[cfg(not(feature = "use_dotstar"))]
pub struct Sleds<SPI> {
    sleds: Arc<Mutex<Ws2812<SPI>>>,
}

impl<SPI: SpiBus> Sleds<SPI> {
    pub fn new(spi_bus: SPI) -> Self
    where
        SPI: SpiBus,
    {
        #[cfg(feature = "use_dotstar")]
        let sleds = Apa102::new(spi_bus);

        #[cfg(not(feature = "use_dotstar"))]
        let sleds = Ws2812::new(spi_bus);

        Self {
            sleds: Arc::new(Mutex::new(sleds)),
        }
    }

    pub fn welcome(&mut self) {
        let mut hue: u16 = 0;
        for i in 0..LED_NUM {
            let mut data = [RGB8::default(); LED_NUM];
            let color = hsv2rgb(Hsv {
                hue: hue as u8,
                sat: 255,
                val: 128, // 32,
            });
            data[i] = color;
            hue = (hue + 256 / LED_NUM as u16) % 256;
            self.sleds
                .lock()
                .unwrap()
                .write(gamma(data.iter().cloned()))
                .unwrap();
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        let mut data = [RGB8::default(); LED_NUM];
        for i in 0..LED_NUM {
            let color = hsv2rgb(Hsv {
                hue: hue as u8,
                sat: 255,
                val: 255, // 128
            });
            data[i] = color;
            hue = (hue + 256 / LED_NUM as u16) % 256;
        }
        self.sleds
            .lock()
            .unwrap()
            .write(gamma(data.iter().cloned()))
            .unwrap();
            std::thread::sleep(std::time::Duration::from_millis(1000));

        // load default hsv
        let (hue, sat, val) = nvs::get_hsv().unwrap();
        info!("hue: {}, sat: {}, val: {}", hue, sat, val);
        *global::LED_HUE.lock().unwrap() = hue;
        *global::LED_SAT.lock().unwrap() = sat;
        *global::LED_VAL.lock().unwrap() = val;

        self.turn_on_all();
    }

    pub fn show_time(&mut self, h: u8, m: u8) {
        let mut h = h;
        let mut m10 = m / 10;
        let mut m1 = m % 10;
        match m1 {
            1 | 2 | 3 => m1 = 0,
            4 | 5 | 6 => m1 = 5,
            7 | 8 | 9 => {
                m1 = 0;
                m10 += 1;
                if m10 == 6 {
                    m10 = 0;
                    h += 1;
                }
            }
            _ => (),
        }

        if (h == 0 || h == 24) && m10 + m1 == 0 {
            self.show_leds(vec![15, 16]); // 자정
            return;
        }

        if h == 12 && m10 + m1 == 0 {
            self.show_leds(vec![16, 21]); // 정오
            return;
        }

        if h > 12 {
            h -= 12;
        }

        let mut leds: Vec<u8> = vec![];

        // start from bottom left
        match h {
            0 | 12 => leds.extend(vec![0, 5, 14]), // 열두시
            1 => leds.extend(vec![1, 14]),         // 한시
            2 => leds.extend(vec![5, 14]),         // 두시
            3 => leds.extend(vec![3, 14]),         // 세시
            4 => leds.extend(vec![4, 14]),         // 네시
            5 => leds.extend(vec![2, 7, 14]),      // 다섯시
            6 => leds.extend(vec![6, 7, 14]),      // 여섯시
            7 => leds.extend(vec![8, 9, 14]),      // 일곱시
            8 => leds.extend(vec![10, 11, 14]),    // 여덟시
            9 => leds.extend(vec![12, 13, 14]),    // 아홉시
            10 => leds.extend(vec![0, 14]),        // 열시
            11 => leds.extend(vec![0, 1, 14]),     // 열한시
            _ => (),
        }
        if m10 + m1 != 0 {
            match m10 {
                1 => leds.extend(vec![22]),     // 십
                2 => leds.extend(vec![17, 19]), // 이십
                3 => leds.extend(vec![18, 19]), // 삼십
                4 => leds.extend(vec![20, 22]), // 사십
                5 => leds.extend(vec![21, 22]), // 오십
                _ => (),
            }
            if m1 == 5 {
                leds.extend(vec![23, 24]); // 오분
            } else {
                leds.extend(vec![24]); // 분
            }
        }
        self.show_leds(leds);
    }

    fn show_leds(&mut self, leds: Vec<u8>) {
        let leds = remap(leds);
        let mut data = [RGB8::default(); LED_NUM];

        let led_hsv = Hsv {
            hue: *global::LED_HUE.lock().unwrap(),
            sat: *global::LED_SAT.lock().unwrap(),
            val: *global::LED_VAL.lock().unwrap(),
        };
        let led_rgb = hsv2rgb(led_hsv);

        for l in leds {
            data[l as usize] = led_rgb;
        }

        self.sleds
            .lock()
            .unwrap()
            .write(gamma(data.iter().cloned()))
            .unwrap();
    }

    pub fn turn_on_all(&mut self) {
        let mut data = [RGB8::default(); LED_NUM];
        let led_hsv = Hsv {
            hue: *global::LED_HUE.lock().unwrap(),
            sat: *global::LED_SAT.lock().unwrap(),
            val: *global::LED_VAL.lock().unwrap(),
        };
        let led_rgb = hsv2rgb(led_hsv);
        for i in 0..LED_NUM {
            data[i] = led_rgb;
        }
        self.sleds.lock().unwrap().write(gamma(data.iter().cloned())).unwrap();
    }
}

fn remap(leds: Vec<u8>) -> Vec<u8> {
    // 0~24 -> 24~0으로 매핑하는 테이블 생성
    let mapping_from_bl_to_top: [u8; 25] = [
        4, 5, 14, 15, 24, 3, 6, 13, 16, 23, 2, 7, 12, 17, 22, 1, 8, 11, 18, 21, 0, 9, 10, 19, 20,
    ];
    leds.into_iter()
        .map(|x| mapping_from_bl_to_top[x as usize])
        .collect()
}
