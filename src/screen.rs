#![allow(unused_imports)]
use core::cell::RefCell;

use defmt::info;

use display_interface_spi::SPIInterface;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::gpio::Output;
use embassy_rp::gpio::{Input, Level, Pull};
use embassy_rp::Peripherals;
use embassy_rp::Peri;
use embassy_rp::peripherals::*;
use embassy_rp::spi::{Blocking, Spi};
use embassy_rp::spi;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;

use embedded_graphics::text::Text;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_10X20, FONT_9X18, FONT_9X18_BOLD};


use st7565::GraphicsPageBuffer;
    // display: ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, embassy_rp::peripherals::SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>

use st7565::ST7565;
use st7565::displays::DOGL128_6;
use st7565::modes::GraphicsMode;


use crate::stack::Stack;

const NAME_LEFT: i32 = 1;
const COLON_LEFT: i32 = 6;
const NUM_LEFT: i32 = 15; 
const LINE_SPACING: i32 = 15;
const X_NUM_BOTTOM: i32 = 62;
const Y_NUM_BOTTOM: i32 = X_NUM_BOTTOM - LINE_SPACING;
const Z_NUM_BOTTOM: i32 = X_NUM_BOTTOM - 2*LINE_SPACING;
const T_NUM_BOTTOM: i32 = X_NUM_BOTTOM - 3*LINE_SPACING;
const X_LABEL_BOTTOM: i32 = 59;
const Y_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - LINE_SPACING;
const Z_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 2*LINE_SPACING;
const T_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 3*LINE_SPACING;


 //on or off makes no difference


        // // Put something on the display so I know its working...:
        // let _= Text::new("x", Point::new(NAME_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        // let _ = Text::new(":", Point::new(COLON_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        // let _ = Text::new("_", Point::new(NUM_LEFT, X_NUM_BOTTOM), font).draw(&mut display);            
        // let _= Text::new("y", Point::new(NAME_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        // let _ = Text::new(":", Point::new(COLON_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        // let _= Text::new("z", Point::new(NAME_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        // let _ = Text::new(":", Point::new(COLON_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        // let _= Text::new("t", Point::new(NAME_LEFT, T_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        // let _ = Text::new(":", Point::new(COLON_LEFT, T_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        // display.flush().unwrap(); 


pub struct Screen<'a> {

    display: &'a ST7565<SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, embassy_rp::peripherals::SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
    font:   MonoTextStyle<'a, BinaryColor>,
    stack_names_font:    &'a MonoTextStyle<'a, BinaryColor>,   
    stack: &'a Stack,
}

impl Screen<'_> {

    
    pub fn new()->Screen<'static>{

        let p = embassy_rp::init(Default::default());

        let mosi = p.PIN_19;
        let miso  = p.PIN_20;
        let display_cs = p.PIN_21;
        let clk = p.PIN_18;
        let reset  = p.PIN_28;
        let a0 = p.PIN_27;

        let mut reset = Output::new(reset, Level::Low);
        let a0 = Output::new(a0, Level::Low);   
        let display_config = spi::Config::default();

        let spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
        let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
        let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
        let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);
        
    info!("display interface created");
        let mut page_buffer = GraphicsPageBuffer::new();
        let mut display = st7565::ST7565::new(display_interface, DOGL128_6)
        .into_graphics_mode(&mut page_buffer);   
 
        display.reset(&mut reset, &mut Delay).unwrap();
        display.flush().unwrap();
        display.set_display_on(true).unwrap();

        let main_font =  FONT_9X18;//PROFONT_14_POINT;//u8g2_font_10x20_tn;// IBM437_9X14_REGULAR;//PROFONT_14_POINT;//
        let stack_names_font = FONT_7X13;


        let dfont = MonoTextStyle::new(&main_font, BinaryColor::On);
        let stack_names_font = MonoTextStyle::new(&stack_names_font, BinaryColor::On);
        let stack = Stack::new();

        // display: &display,

        Screen { 
            display: &display,
            font: dfont,
            stack_names_font: &stack_names_font,
            stack: &stack,
        }
    }



    pub fn update(&mut self){
        let stack = self.stack.fetch_strs();

        let x = stack.0;

        self.display.clear(BinaryColor::Off);
        let _= Text::new("x", Point::new(NAME_LEFT, X_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, X_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(stack.0, Point::new(NUM_LEFT, X_NUM_BOTTOM), self.font).draw(&mut self.display);
        let _= Text::new("y", Point::new(NAME_LEFT, Y_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Y_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(stack.1, Point::new(NUM_LEFT, Y_NUM_BOTTOM), self.font).draw(&mut self.display);
        let _= Text::new("z", Point::new(NAME_LEFT, Z_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Z_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(stack.2, Point::new(NUM_LEFT, Z_NUM_BOTTOM), self.font).draw(&mut self.display);
        let _= Text::new("t", Point::new(NAME_LEFT, T_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(":", Point::new(COLON_LEFT, T_LABEL_BOTTOM), self.stack_names_font).draw(&mut self.display);
        let _ = Text::new(stack.3, Point::new(NUM_LEFT, T_NUM_BOTTOM), self.font).draw(&mut self.display);
        self.display.flush().unwrap();
    }
}

            
           