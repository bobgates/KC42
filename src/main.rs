#![no_std]
#![no_main]

// 21 May 2026: In this version, the screen and keyboard work and I've started 
//      looking at command processing.
//
// Plan: get the calculator crate to return an enum of Styles, and 
// then just print them to the screen here.

mod keyboard;
// use embassy_rp::peripherals::SPI0;
use keyboard::Keyboard;

// use st7565::modes::GraphicsMode;
// mod screen;
// use screen::{Screen};

use st7565::{GraphicsPageBuffer};
use st7565::displays::DOGL128_6;
// use st7565::modes::GraphicsMode;
// mod logic;

// mod types;
use core::cell::RefCell;

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

    // let display_spi: SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
    let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);
    
   
    let mut page_buffer = GraphicsPageBuffer::new();
    let mut display = st7565::ST7565::new(display_interface, DOGL128_6)
        .into_graphics_mode(&mut page_buffer);   
    display.reset(&mut reset, &mut Delay).unwrap();
    display.flush().unwrap();
    display.set_display_on(true).unwrap();

    // let mut screen = Screen::new(&mut display);

    // screen.draw().unwrap();
    let circle =Circle::new(Point::new(50, 50), 20)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2));
    
    circle.draw( &mut display);
    let rectangle =Rectangle::new(Point::new(106, 6), Size::new(20, 20))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2));
    rectangle.draw(&mut display);
    let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

    // 10x20 (which is 8x13) - good size for 4 lines, probably just a good size 
    let _ =Text::new("11.1111345", Point::new(0, 13), font)
        .draw(&mut display);
    let _ =Text::new("123.4567", Point::new(0, 29), font)
        .draw(&mut display);
    let _ =Text::new("34.5678", Point::new(3, 45), font)
        .draw(&mut display);
    let _ =Text::new("12.34321", Point::new(3, 61), font)
        .draw(&mut display);
    // Text::new("56.789", Point::new(3, 62), font)
    //     .draw(&mut self.display)
    //     .unwrap();


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


    let _ =display.flush();
    let mut keyboard = Keyboard::new(rows, cols);
    loop {

        let key = keyboard.scan().await;
        // if key.is_some() {
            // info!("In main, Key {:?} pressed", key);
        // }
        Timer::after_millis(10).await; 
        match key {
            Some(key) => {    
                match key {
                    keyboard::KeyName::Number(n) => info!("Key {:?} pressed", n),
                    _ =>   info!("In main, Key {:?} pressed", key),
                }
                // println!("Key {:?} pressed", k);
            }
            _ => {  // default case, when no key is pressed
                // error!("Error in keypress processing");
            }
        }


        let _ =Text::new("12.34321", Point::new(3, 61), font)
        .draw(&mut display);

    }
}







// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     let p = embassy_rp::init(Default::default());
//     info!("Started");

//     // let mut led = Output::new(p.PIN_25, Level::Low);

//     let mosi = p.PIN_19;
//     let miso  = p.PIN_20;
//     let display_cs = p.PIN_21;
//     let clk = p.PIN_18;
//     let reset  = p.PIN_28;
//     let a0 = p.PIN_27;

//     let mut reset = Output::new(reset, Level::Low);
//     let a0 = Output::new(a0, Level::Low);   

//     let display_config = spi::Config::default();

//     static spi<T,M>:Spi = Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
//     static spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

//     // let display_spi: SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
//     let display_spi=SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs, Level::High), display_config);
//     let display_interface: SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>, Output<'_>> = SPIInterface::new(display_spi, a0);
    
   
//     let mut page_buffer = GraphicsPageBuffer::new();
//     let mut display = st7565::ST7565::new(display_interface, DOGL128_6)
//         .into_graphics_mode(&mut page_buffer);   
//     display.reset(&mut reset, &mut Delay).unwrap();
//     display.flush().unwrap();
//     display.set_display_on(true).unwrap();


//     // Keyboard pins
//     let row1 = Input::new(p.PIN_2, Pull::Down);
//     let row2 = Input::new(p.PIN_3, Pull::Down);
//     let row3 = Input::new(p.PIN_4, Pull::Down);
//     let row4 = Input::new(p.PIN_5, Pull::Down);
//     let row5 = Input::new(p.PIN_6, Pull::Down);
//     let row6 = Input::new(p.PIN_7, Pull::Down);
//     let row7 = Input::new(p.PIN_8, Pull::Down);
//     let row8 = Input::new(p.PIN_9, Pull::Down);


//     let col1 = Output::new(p.PIN_10, Level::Low); 
//     let col2 = Output::new(p.PIN_11, Level::Low);
//     let col3 = Output::new(p.PIN_12, Level::Low);
//     let col4 = Output::new(p.PIN_13, Level::Low);
//     let col5 = Output::new(p.PIN_14, Level::Low);
//     let col6 = Output::new(p.PIN_15, Level::Low);

//     let rows = [row1, row2, row3, row4, row5, row6, row7, row8];
//     let cols = [col1, col2, col3, col4, col5, col6];

//     let mut keyboard = Keyboard::new(rows, cols);

//     info!("Keyboard initialized");

//     let mut display_config = spi::Config::default();
//     display_config.frequency = DISPLAY_FREQ;
//     display_config.phase = spi::Phase::CaptureOnSecondTransition;
//     display_config.polarity = spi::Polarity::IdleHigh;


//     let mut screen = Screen::new(&mut display);


//     screen.draw().unwrap();



//     //     // Draw content
//     //     let _ =Circle::new(Point::new(106, 106), 20)
//     //         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
//     //         .draw(&mut display);
//     //     let _ =Rectangle::new(Point::new(106, 6), Size::new(20, 20))
//     //         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
//     //         .draw(&mut display);
//     //     let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);


//     //     // 10x20 (which is 8x13) - good size for 4 lines, probably just a good size 
//     //     let _ =Text::new("11.1111345", Point::new(0, 13), font)
//     //         .draw(&mut display);
//     //     let _ =Text::new("23.4567", Point::new(0, 29), font)
//     //         .draw(&mut display);
//     //     let _ =Text::new("34.5678", Point::new(3, 45), font)
//     //         .draw(&mut display);
//     //     let _ =Text::new("88.8888", Point::new(3, 61), font)
//     //         .draw(&mut display);
//     //     // Text::new("56.789", Point::new(3, 62), font)
//     //     //     .draw(&mut display)
//     //     //     .unwrap();


//     //     let _ =display.flush();
//     // //        let _ =Circle::new(Point::new(106, 106), 20)
//     // //         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
//     // //         .draw(disp);
//     // // //  draw(disp).unwrap();


//     loop {
//         let key = keyboard.scan().await;
//         if key.is_some() {
//             info!("In main, Key {:?} pressed", key);
//         }
//         Timer::after_millis(10).await; 
//     }             
// }

// pub struct DrawStack<'a>{
//     disp: ST7565<SPIInterface<SpiDeviceWithConfig<'a, NoopRawMutex, Spi<'a, embassy_rp::peripherals::SPI0, Blocking>, Output<'a>>, Output<'a>>, DOGL128_6, GraphicsMode<'a, 128, 8>, 128, 64, 8>,
//     x: f64,
//     y: f64,
//     z: f64,
//     a: f64,
// }

// impl DrawStack {
//     pub fn new() -> Self {
//         Self {
//             disp: ST7565::new(),
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//             a: 0.0,
//         }
//     }
// }








// pub fn draw(mut disp: ST7565<SPIInterface<SpiDeviceWithConfig<'_, NoopRawMutex, Spi<'_, SPI0, Blocking>, Output<'_>>,
//          Output<'_>>, DOGL128_6, GraphicsMode<'_, 128, 8>, 128, 64, 8>)-> Result<(), core::convert::Infallible >
// {
//     use embedded_graphics::prelude::*;
//     use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
//     use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
//     use embedded_graphics::text::Text;
//     use embedded_graphics::pixelcolor::BinaryColor;

//     // Draw content
//     let _ =Circle::new(Point::new(106, 106), 20)
//         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
//         .draw(&mut disp);
//     let _ =Rectangle::new(Point::new(106, 6), Size::new(20, 20))
//         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
//         .draw(&mut disp)?;
//     let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

//     // 10x20 (which is 8x13) - good size for 4 lines, probably just a good size 
//     let _ =Text::new("12.3456", Point::new(0, 13), font)
//         .draw(&mut disp)?;
//     let _ =Text::new("23.4567", Point::new(0, 29), font)
//         .draw(&mut disp);
//     Text::new("34.5678", Point::new(3, 45), font)
//         .draw(&mut disp)?;
//     let _ =Text::new("45.6789", Point::new(3, 61), font)
//         .draw(&mut disp)?;
//     // Text::new("56.789", Point::new(3, 62), font)
//     //     .draw(&mut disp)
//     //     .unwrap();


//     let _ =disp.flush();
//     Ok(())
// }