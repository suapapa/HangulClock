mod dotstar;
mod neopixel;

#[cfg(feature = "use_dotstar")]
use apa102_spi::Apa102;
#[cfg(not(feature = "use_dotstar"))]
use ws2812_spi::Ws2812;

use embedded_hal::spi::SpiBus;

// const LED_NUM: usize = 25;
// const DEFAULT_BRIGHTNESS: u8 = 100;

#[cfg(feature = "use_dotstar")]
pub struct Sleds<SPI> {
    sleds: Apa102<SPI>,
}

#[cfg(not(feature = "use_dotstar"))]
pub struct Sleds<SPI> {
    sleds: Ws2812<SPI>,
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

        Self { sleds }
    }

    pub fn welcome(&mut self) {
        #[cfg(feature = "use_dotstar")]
        dotstar::welcome(&mut self.sleds);
        #[cfg(not(feature = "use_dotstar"))]
        neopixel::welcome(&mut self.sleds);
    }

    pub fn show_time(&mut self, h: u8, m: u8) {
        #[cfg(feature = "use_dotstar")]
        dotstar::show_time(&mut self.sleds, h, m);
        #[cfg(not(feature = "use_dotstar"))]
        neopixel::show_time(&mut self.sleds, h, m);
    }
}
