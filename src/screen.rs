// use core::cell::RefCell;

// use embassy_rp::Peri;//, peripherals::PIN_21};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
// use embassy_sync::blocking_mutex::Mutex;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::gpio::Output; //{Level, Output};
// use embassy_rp::peripherals::SPI0;
use embassy_rp;
// use embassy_rp::spi;
use embassy_rp::spi::{Blocking, Spi};
// use embassy_time::Delay;
// use embedded_graphics::pixelcolor::BinaryColor;
//use embedded_graphics::primitives::{Circle, Primitive, PrimitiveStyle, Rectangle};
// use embedded_graphics::Drawable;
// use embedded_graphics::text::Text;
// use embedded_graphics::geometry::{Point, Size};
// use embedded_graphics::mono_font::MonoTextStyle;
// use embedded_graphics::mono_font::ascii::{ FONT_10X20};//FONT_9X18};
// use embedded_graphics::mono_font::{FONT_9X15, FONT_9X18_BOLD, FONT_10X20, FONT_8X13, MonoFont};
// 10x20 is 8x13
// 9x18 is 7x10
use st7565::ST7565;
use display_interface_spi::SPIInterface;
// use st7565::GraphicsPageBuffer;
use st7565::modes::GraphicsMode;
use st7565::displays::DOGL128_6;

const DISPLAY_FREQ: u32 = 20_000_000;        //DOGL128   supports up to 4MHz, but it seems to cause issues.

pub struct Car{
    display: usize,
}

impl Car {
    pub fn new() -> Self {
        Car {
            display: 0,
        }
    }       
}


pub struct Screen<'a> {
    display: &'a mut ST7565::<SPIInterface<SpiDeviceWithConfig<'a, NoopRawMutex, Spi<'a, embassy_rp::peripherals::SPI0, Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>
}

impl Screen<'static> {
    pub fn new(display: &'static mut ST7565<SPIInterface<SpiDeviceWithConfig<'static, NoopRawMutex, Spi<'static, embassy_rp::peripherals::SPI0, Blocking>, Output<'static>>, Output<'static>>, DOGL128_6, GraphicsMode<'static, 128, 8>, 128, 64, 8>) -> Self {
        Screen::<'static> {
            display: display,  
        }
    }

    pub fn draw(&mut self,)-> Result<(), core::convert::Infallible >{// mut disp: ST7565<SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>,
            // Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8>)-> Result<(), core::convert::Infallible >
    
        use embedded_graphics::prelude::*;
        use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
        use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
        use embedded_graphics::text::Text;
        use embedded_graphics::pixelcolor::BinaryColor;

        // Draw content
        let _ =Circle::new(Point::new(106, 106), 20)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
            .draw( self.display);
        let _ =Rectangle::new(Point::new(106, 6), Size::new(20, 20))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
            .draw(self.display)?;
        let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

        // 10x20 (which is 8x13) - good size for 4 lines, probably just a good size 
        let _ =Text::new("11.1111345", Point::new(0, 13), font)
            .draw(self.display)?;
        let _ =Text::new("123.4567", Point::new(0, 29), font)
            .draw(self.display);
        Text::new("34.5678", Point::new(3, 45), font)
            .draw(self.display)?;
        let _ =Text::new("88.8888", Point::new(3, 61), font)
            .draw(self.display)?;
        // Text::new("56.789", Point::new(3, 62), font)
        //     .draw(&mut self.display)
        //     .unwrap();


        let _ =self.display.flush();
        Ok(())
    }
}

