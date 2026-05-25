#![no_std]
#![no_main]

// 21 May 2026: In this version, the screen and keyboard work and I've started 
//      looking at command processing.
//
// Plan: get the calculator crate to return an enum of Styles, and 
// then just print them to the screen here.

mod keyboard;
use heapless::{format, String};
use embedded_graphics::mono_font::ascii::FONT_7X13;
use embedded_graphics::mono_font::iso_8859_2::FONT_5X8;
// use embassy_rp::peripherals::SPI0;
use keyboard::Keyboard;
// use keyboard::KeyName;
mod calc;
// use calc;

// use st7565::modes::GraphicsMode;
// mod screen;
// use screen::{Screen};

use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;
// use st7565::modes::GraphicsMode;
// mod logic;

// mod types;
use core::cell::RefCell;
use crate::calc::Calc;

use defmt::*;
use display_interface_spi::SPIInterface;
// use embassy_rp as hal;
use embassy_rp::gpio::Output;
use embassy_rp::gpio::{Input, Level, Pull};
// use embassy_rp::{Peri, Peripherals};
use embassy_executor::Spawner;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::spi::{Blocking, Spi};
use embassy_rp::spi;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
// use embassy_sync::channel::Channel;
use embassy_time::Delay;
use embassy_time::Timer;
use embassy_rp::peripherals::{ SPI0};
//PIN_18, PIN_19, PIN_20, PIN_21, PIN_27, PIN_28,

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_graphics::pixelcolor::BinaryColor;

use {defmt_rtt as _, panic_probe as _};
// use st7565::types::{BoosterRatio, PowerControlMode};
// use embedded_hal::blocking::spi::Transfer;

// use types::DisplaySpecs;
// const DISPLAY_FREQ: u32 = 20_000_000;  

const NAME_LEFT: i32 = 1;
const COLON_LEFT: i32 = 6;
const NUM_LEFT: i32 = 12; 
const LINE_SPACING: i32 = 15;
const X_NUM_BOTTOM: i32 = 62;
const Y_NUM_BOTTOM: i32 = X_NUM_BOTTOM - LINE_SPACING;
const Z_NUM_BOTTOM: i32 = X_NUM_BOTTOM - 2*LINE_SPACING;
const A_NUM_BOTTOM: i32 = X_NUM_BOTTOM - 3*LINE_SPACING;
const X_LABEL_BOTTOM: i32 = 59;
const Y_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - LINE_SPACING;
const Z_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 2*LINE_SPACING;
const A_LABEL_BOTTOM: i32 = X_LABEL_BOTTOM - 3*LINE_SPACING;


// (1, 59-ls), stack_font).draw(&mut display);
//                 let _ = Text::new(":", Point::new(6, 59-ls), stack_font).draw(&mut display);
//                 let _ = Text::new(&text, Point::new(12, 62-ls),



#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Started");

    // let mut led = Output::new(p.PIN_25, Level::Low);

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
    
   
    let mut page_buffer = GraphicsPageBuffer::new();
    let mut display = st7565::ST7565::new(display_interface, DOGL128_6)
        .into_graphics_mode(&mut page_buffer);   
    display.reset(&mut reset, &mut Delay).unwrap();
    display.flush().unwrap();
    display.set_display_on(true).unwrap();

    // let circle =Circle::new(Point::new(50, 50), 20)
    //     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2));
    // circle.draw(&mut display);

    let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let stack_names_font = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);


    // Keyboard pins
    let row1 = Input::new(p.PIN_2, Pull::Down);
    let row2 = Input::new(p.PIN_3, Pull::Down);
    let row3 = Input::new(p.PIN_4, Pull::Down);
    let row4 = Input::new(p.PIN_5, Pull::Down);
    let row5 = Input::new(p.PIN_6, Pull::Down);
    let row6 = Input::new(p.PIN_7, Pull::Down);
    let row7 = Input::new(p.PIN_8, Pull::Down);
    let row8 = Input::new(p.PIN_9, Pull::Down);


    let col1 = Output::new(p.PIN_10, Level::Low); 
    let col2 = Output::new(p.PIN_11, Level::Low);
    let col3 = Output::new(p.PIN_12, Level::Low);
    let col4 = Output::new(p.PIN_13, Level::Low);
    let col5 = Output::new(p.PIN_14, Level::Low);
    let col6 = Output::new(p.PIN_15, Level::Low);

    let rows = [row1, row2, row3, row4, row5, row6, row7, row8];
    let cols = [col1, col2, col3, col4, col5, col6];

    unsafe {
        let mut calc =  Calc::new();
        let mut keyboard = Keyboard::new(rows, cols);

        // Put something on the display so I know its working...:
        let _= Text::new("x", Point::new(NAME_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(":", Point::new(COLON_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _= Text::new("y", Point::new(NAME_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _= Text::new("z", Point::new(NAME_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _= Text::new("a", Point::new(NAME_LEFT, A_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(":", Point::new(COLON_LEFT, A_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        display.flush().unwrap();        

        loop {
            let key = keyboard.scan().await;
            if key == None {
                Timer::after_millis(10).await; 
                continue;
            }
            info!("key arrived");
            // let ls=15; // line spacing
            let (stacky, stackz, stacka)=calc.update_stack_display();
            let ytext: String<64> = format!("{}", stacky).unwrap();
            let ztext: String<64> = format!("{}", stackz).unwrap();
            let atext: String<64> = format!("{}", stacka).unwrap();

            // let ztext = format!("{?}", stackz);
            // let atext = format!("{?}", stacka);
            
            if let Some(xtext) = calc.input_key(key){
                 display.clear(BinaryColor::Off);
                let _= Text::new("x", Point::new(NAME_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
                let _ = Text::new(":", Point::new(COLON_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
                let _ = Text::new(&xtext, Point::new(NUM_LEFT, X_NUM_BOTTOM), font).draw(&mut display);
                let _= Text::new("y", Point::new(NAME_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
                let _ = Text::new(":", Point::new(COLON_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
                let _ = Text::new(&ytext, Point::new(NUM_LEFT, Y_NUM_BOTTOM), font).draw(&mut display);
                let _= Text::new("z", Point::new(NAME_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
                let _ = Text::new(":", Point::new(COLON_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
                let _ = Text::new(&ztext, Point::new(NUM_LEFT, Z_NUM_BOTTOM), font).draw(&mut display);
                let _= Text::new("a", Point::new(NAME_LEFT, A_LABEL_BOTTOM), stack_names_font).draw(&mut display);
                let _ = Text::new(":", Point::new(COLON_LEFT, A_LABEL_BOTTOM), stack_names_font).draw(&mut display);
                let _ = Text::new(&atext, Point::new(NUM_LEFT, A_NUM_BOTTOM), font).draw(&mut display);
                
                // info!("inside display clear and x code");
            }
display.flush().unwrap();



            //     // DONT ALLOW POINTS AFTER E             
            // }



        }
}
}



