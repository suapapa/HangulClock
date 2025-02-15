#[cfg(feature = "use_dotstar")]
use apa102_spi::Apa102;
#[cfg(not(feature = "use_dotstar"))]
use ws2812_spi::Ws2812;

use embedded_hal::spi::SpiBus;
use smart_leds::{gamma, hsv::hsv2rgb, hsv::Hsv, SmartLedsWrite, RGB8};

use std::sync::{Arc, Mutex};

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
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        let mut data = [RGB8::default(); LED_NUM];
        for i in 0..LED_NUM {
            let color = hsv2rgb(Hsv {
                hue: hue as u8,
                sat: 255,
                val: 128, // 32,
            });
            data[i] = color;
            hue = (hue + 256 / LED_NUM as u16) % 256;
        }
        self.sleds
            .lock()
            .unwrap()
            .write(gamma(data.iter().cloned()))
            .unwrap();
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
            self.show_leds(vec![9, 8]); // 자정
            return;
        }

        if h == 12 && m10 + m1 == 0 {
            self.show_leds(vec![8, 1]); // 정오
            return;
        }

        if h > 12 {
            h -= 12;
        }

        let mut leds: Vec<u8> = vec![];
        /*
        // start from top right
        match h {
            0 | 12 => leds.extend(vec![4, 5, 10]), // 열두시
            1 => leds.extend(vec![3, 10]),         // 한시
            2 => leds.extend(vec![5, 10]),         // 두시
            3 => leds.extend(vec![1, 10]),         // 세시
            4 => leds.extend(vec![0, 10]),         // 네시
            5 => leds.extend(vec![2, 7, 10]),      // 다섯시
            6 => leds.extend(vec![6, 7, 10]),      // 여섯시
            7 => leds.extend(vec![8, 9, 10]),      // 일곱시
            8 => leds.extend(vec![14, 13, 10]),    // 여덟시
            9 => leds.extend(vec![12, 11, 10]),    // 아홉시
            10 => leds.extend(vec![4, 10]),        // 열시
            11 => leds.extend(vec![4, 3, 10]),     // 열한시
            _ => (),
        }
        if m10 + m1 != 0 {
            match m10 {
                1 => leds.extend(vec![19]),     // 십
                2 => leds.extend(vec![17, 22]), // 이십
                3 => leds.extend(vec![18, 19]), // 삼십
                4 => leds.extend(vec![24, 22]), // 사십
                5 => leds.extend(vec![23, 22]), // 오십
                _ => (),
            }
            if m1 == 5 {
                leds.extend(vec![21, 20]); // 오분
            } else {
                leds.extend(vec![20]); // 분
            }
        }
        */

        // start from bottom left
        match h {
            0 | 12 => leds.extend(vec![20, 19, 14]), // 열두시
            1 => leds.extend(vec![21, 14]),          // 한시
            2 => leds.extend(vec![19, 14]),          // 두시
            3 => leds.extend(vec![23, 14]),          // 세시
            4 => leds.extend(vec![24, 14]),          // 네시
            5 => leds.extend(vec![22, 17, 14]),      // 다섯시
            6 => leds.extend(vec![18, 17, 14]),      // 여섯시
            7 => leds.extend(vec![16, 15, 14]),      // 일곱시
            8 => leds.extend(vec![10, 11, 14]),      // 여덟시
            9 => leds.extend(vec![12, 13, 14]),      // 아홉시
            10 => leds.extend(vec![20, 14]),         // 열시
            11 => leds.extend(vec![20, 21, 14]),     // 열한시
            _ => (),
        }
        if m10 + m1 != 0 {
            match m10 {
                1 => leds.extend(vec![5]),    // 십
                2 => leds.extend(vec![7, 2]), // 이십
                3 => leds.extend(vec![6, 5]), // 삼십
                4 => leds.extend(vec![0, 2]), // 사십
                5 => leds.extend(vec![1, 2]), // 오십
                _ => (),
            }
            if m1 == 5 {
                leds.extend(vec![3, 4]); // 오분
            } else {
                leds.extend(vec![4]); // 분
            }
        }
        self.show_leds(leds);
    }

    fn show_leds(&mut self, leds: Vec<u8>) {
        let mut data = [RGB8::default(); LED_NUM];
        for l in leds {
            data[l as usize] = RGB8 {
                r: 0x65, // DEFAULT_BRIGHTNESS,
                g: 0x5b, // DEFAULT_BRIGHTNESS,
                b: 0xf9, // DEFAULT_BRIGHTNESS,
            };
        }

        self.sleds
            .lock()
            .unwrap()
            .write(gamma(data.iter().cloned()))
            .unwrap();
    }
}
