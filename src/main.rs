#![no_std]
#![no_main]

// 21 May 2026: In this version, the screen and keyboard work and I've started 
//      looking at command processing.
//
// Plan: get the calculator crate to return an enum of Styles, and 
// then just print them to the screen here.

mod calc;
use crate::calc::{Calc};//, Stack, string_to_number, convert_to_string};
mod stack;

use core::cell::RefCell;
// use core::mem::MaybeUninit;

use defmt::*;

use display_interface_spi::SPIInterface;

use embassy_rp::gpio::Output;
use embassy_rp::gpio::{Input, Level, Pull};
// use embassy_rp::{Peri, Peripherals};
use embassy_executor::Spawner;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::spi::{Blocking, Spi};
use embassy_rp::spi;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
use embassy_time::Timer;
use embassy_rp::peripherals::{ SPI0};

use embedded_graphics::{prelude::*};//, text};
// use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::mono_font::ascii::FONT_7X13;

use heapless::{String};//, Vec};
// use heapless::vec::VecInner;
 
mod keyboard;
use keyboard::Keyboard;
use heapless::format;

use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;


use {defmt_rtt as _, panic_probe as _};

// use types::DisplaySpecs;
// const DISPLAY_FREQ: u32 = 20_000_000;  

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





#[embassy_executor::main]
async fn main_(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Started");

    // let mut led = Output::new(p.PIN_25, Level::Low);

    let mosi = p.PIN_19;
    let miso  = p.PIN_20;
    let display_cs = p.PIN_21;
    let clk = p.PIN_18;
    let reset  = p.PIN_28;
    let a0 = p.PIN_27;

    // let x_buffer : &str ="";
    // let y_buffer : &str ="";
    // let z_buffer : &str ="";
    // let t_buffer : &str ="";

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

    // Need to create num_buffer here, pass it in to the keyboard reader,
    // then get it back from the keyboard reader due to borrowing rules
    // and nostd
    // let mut num_buffer: Vec<u8, 64> = Vec::new();
    // num_buffer.push('_' as u8).expect("Failed to push '_' into num_buffer in main()");


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

    let _buffer_string = 

    unsafe {
        let mut calc =  Calc::new();
        let mut keyboard = Keyboard::new(rows, cols);

        // This is to put the '_' on the screen on startup
        let num_buffer_str = "_";//calc.process_key(key).unwrap();


        // Put something on the display so I know its working...:
        let _= Text::new("x", Point::new(NAME_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(":", Point::new(COLON_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(&num_buffer_str, Point::new(NUM_LEFT, X_NUM_BOTTOM), font).draw(&mut display);            
        let _= Text::new("y", Point::new(NAME_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _= Text::new("z", Point::new(NAME_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(":", Point::new(COLON_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _= Text::new("t", Point::new(NAME_LEFT, T_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        let _ = Text::new(":", Point::new(COLON_LEFT, T_LABEL_BOTTOM), stack_names_font).draw(&mut display);
        display.flush().unwrap(); 

        loop {
            let key = keyboard.scan().await;
            if key == None {
                Timer::after_millis(10).await; 
                continue;
            }

            // Gets updated and later printed for each character entered
            let x_buffer_str = calc.process_key(key).unwrap();



// Could optimise to only update most of display only when stack changes, but not yet. (POITROAE)
            // if calc.stack.changed(){
            //     info!("Stack changed");
            // };

            let (_x_val, y_val, z_val, t_val) = calc.stack.fetch_values();
            // info!("x buffer: {}, x: {}, y: {}, z: {} t: {}",x_buffer_str, x_val, y_val, z_val, t_val);

            // let x_str: String<64> = format!("{:e}", x_val).expect("failed to convert number_to_string ");
            let y_str: String<64> = format!("{:e}", y_val).expect("failed to convert number_to_string ");
            let z_str: String<64> = format!("{:e}", z_val).expect("failed to convert number_to_string ");
            let t_str: String<64> = format!("{:e}", t_val).expect("failed to convert number_to_string ");

            
            display.clear(BinaryColor::Off); //on or off makes no difference
            let _= Text::new("x", Point::new(NAME_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
            let _ = Text::new(":", Point::new(COLON_LEFT, X_LABEL_BOTTOM), stack_names_font).draw(&mut display);
            let _ = Text::new(&x_buffer_str, Point::new(NUM_LEFT, X_NUM_BOTTOM), font).draw(&mut display);
            let _= Text::new("y", Point::new(NAME_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
            let _ = Text::new(":", Point::new(COLON_LEFT, Y_LABEL_BOTTOM), stack_names_font).draw(&mut display);
            let _ = Text::new(&y_str, Point::new(NUM_LEFT, Y_NUM_BOTTOM), font).draw(&mut display);
            let _= Text::new("z", Point::new(NAME_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
            let _ = Text::new(":", Point::new(COLON_LEFT, Z_LABEL_BOTTOM), stack_names_font).draw(&mut display);
            let _ = Text::new(&z_str, Point::new(NUM_LEFT, Z_NUM_BOTTOM), font).draw(&mut display);
            let _= Text::new("t", Point::new(NAME_LEFT, T_LABEL_BOTTOM), stack_names_font).draw(&mut display);
            let _ = Text::new(":", Point::new(COLON_LEFT, T_LABEL_BOTTOM), stack_names_font).draw(&mut display);
            let _ = Text::new(&t_str, Point::new(NUM_LEFT, T_NUM_BOTTOM), font).draw(&mut display);
            
            display.flush().unwrap();
            // info!("End of key loop");
        }
    };
}





