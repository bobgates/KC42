#![no_std]
#![no_main]

mod keyboard;
// use embassy_rp::peripherals::SPI0;
use keyboard::Keyboard;

// use st7565::modes::GraphicsMode;
mod screen;
use screen::{Screen};

use st7565::{GraphicsPageBuffer};//, ST7565};
use st7565::displays::DOGL128_6;
// use st7565::modes::GraphicsMode;
// mod logic;

// mod types;
// use core::cell::RefCell;

use defmt::*;
use display_interface_spi::SPIInterface;
// use embassy_rp as hal;
use embassy_rp::gpio::Output;
use embassy_rp::gpio::{Input, Level, Pull};
use embassy_rp::{Peri};//, Peripherals};
use embassy_executor::Spawner;
// use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::spi::{Blocking, Spi};
use embassy_rp::spi;
// use embassy_sync::blocking_mutex::Mutex;
// use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
// use embassy_time::Timer;
// use rp235x_hal::Timer;
use embassy_rp::peripherals::{PIN_18, PIN_19, PIN_20, PIN_21, PIN_27, PIN_28, SPI0};

        // use embedded_graphics::prelude::*;
        // use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
        // use embedded_graphics::mono_font;//::{ascii::FONT_10X20, MonoTextStyle};
        // use embedded_graphics::text::Text;
        // use embedded_graphics::pixelcolor::BinaryColor;
        // use embedded_hal_compat::{ForwardCompat, ReverseCompat}; // also spi::Compat as SpiCompat};

use {defmt_rtt as _, panic_probe as _};
// use st7565::types::{BoosterRatio, PowerControlMode};
// use embedded_hal::blocking::spi::Transfer;

// use types::DisplaySpecs;
const DISPLAY_FREQ: u32 = 20_000_000;  

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // let a: u32 = p.SPIO;
    // info!("Hello World!");

    // static p=p;

    // Display pins
    let mosi: Peri<'_, PIN_19> = p.PIN_19;
    let miso: Peri<'_, PIN_20> = p.PIN_20;
    let display_cs: Peri<'_, PIN_21> = p.PIN_21;
    let clk: Peri<'_, PIN_18> = p.PIN_18;
    let reset: Peri<'_, PIN_28> = p.PIN_28;
    let a0: Peri<'_, PIN_27> = p.PIN_27;

    // embassy_hal_internal::peripheral::Peri
// impl<'a, T> Peri<'a, T>
// pub fn into<U>(self) -> Peri<'a, U>
// where
//     T: Into<U>,
//     U: PeripheralType,
//     // Bounds from impl:
//     T: PeripheralType,
// T = PIN_18

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

    let mut keyboard = Keyboard::new(rows, cols);

    info!("Keyboard initialized");

    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;

    // let spi: embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking> = Spi::new_blocking(
    let spi: embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking> = Spi::new_blocking(
        p.SPI0.into(),
        clk.into::<PIN_18>(),
        mosi.into::<PIN_19>(),
        miso.into::<PIN_20>(),
        display_config.clone(),
    );

    // let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    // let display_spi = SpiDeviceWithConfig::new(&spi_bus, Output::new(display_cs.into::<PIN_21>(), Level::High), display_config);

// required by a bound in `st7565::driver::mode_initial::<impl ST7565<DI, SPECS, InitialMode, WIDTH, HEIGHT, PAGES>>::new`
// let display_interface: SPIInterface<Spi<'_, SPI0, Blocking>, Output<'_>>;
    
    let a0: Output<'_> = Output::new(a0, embassy_rp::gpio::Level::Low);
    let mut reset = Output::new(reset, embassy_rp::gpio::Level::Low);    
    let display_interface: SPIInterface<Spi<'_, embassy_rp::peripherals::SPI0, Blocking>, Output<'_>> = SPIInterface::new(spi, a0);
    // let display_interface: SPIInterface<Spi<'_, SPI0, Blocking>, Output<'_>> = SPIInterface::new(spi, a0);

    static mut PAGE_BUFFER: GraphicsPageBuffer<128, 8> = GraphicsPageBuffer::new();
    let page_buffer = unsafe { &mut PAGE_BUFFER };
    let mut display = st7565::ST7565::new(display_interface, DOGL128_6)
//     the trait bound `embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>: embedded_hal::spi::SpiDevice` is not satisfied
// you can use `cargo tree` to explore your dependency tree
// required for `SPIInterface<embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>` to implement
// `display_interface::WriteOnlyDataCommand`rustcClick for full compiler diagnostic
//
//  required by a bound in `st7565::driver::mode_initial::<impl ST7565<DI, SPECS, InitialMode, WIDTH, HEIGHT, PAGES>>::new`

//.                    ---------------------
// main.rs(127, 23): required by a bound introduced by this call
// spi.rs(347, 1): there are multiple different versions of crate `embedded_hal` in the dependency graph
// mode_initial.rs(17, 9): required by a bound in `st7565::driver::mode_initial::<impl ST7565<DI, SPECS, InitialMode, WIDTH, HEIGHT, PAGES>>::new`
    //the trait embedded_hal::spi::SpiDevice` is not implemented for `embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking> -- display_interface

        .into_graphics_mode(page_buffer);

// the method `into_graphics_mode` exists for struct `ST7565<SPIInterface<Spi<'_, SPI0, ...>, ...>, ..., ..., 128, 64, 8>`, but its trait bounds were not satisfied
// the following trait bounds were not satisfied:
// `embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>: embedded_hal::spi::SpiDevice`
// which is required by `SPIInterface<embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>: display_interface::WriteOnlyDataCommand`

    display.reset(&mut reset, &mut Delay).unwrap();
    display.flush().unwrap();
    display.set_display_on(true).unwrap();

    let mut screen = Screen::new(&mut display);


    screen.draw().unwrap();

//    
//  pub trait SpiDevice<Word: Copy + 'static = u8>: ErrorType
// ST7565<SPIInterface<embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>, Output<'_>>, DOGL128_6, InitialMode, 128, 64, 8>

    //     // Draw content
    //     let _ =Circle::new(Point::new(106, 106), 20)
    //         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
    //         .draw(&mut display);
    //     let _ =Rectangle::new(Point::new(106, 6), Size::new(20, 20))
    //         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
    //         .draw(&mut display);
    //     let font = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);


    //     // 10x20 (which is 8x13) - good size for 4 lines, probably just a good size 
    //     let _ =Text::new("11.1111345", Point::new(0, 13), font)
    //         .draw(&mut display);
    //     let _ =Text::new("23.4567", Point::new(0, 29), font)
    //         .draw(&mut display);
    //     let _ =Text::new("34.5678", Point::new(3, 45), font)
    //         .draw(&mut display);
    //     let _ =Text::new("88.8888", Point::new(3, 61), font)
    //         .draw(&mut display);
    //     // Text::new("56.789", Point::new(3, 62), font)
    //     //     .draw(&mut display)
    //     //     .unwrap();


    //     let _ =display.flush();
    // //        let _ =Circle::new(Point::new(106, 106), 20)
    // //         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
    // //         .draw(disp);
    // // //  draw(disp).unwrap();


    loop {
        let key = keyboard.scan().await;
        if key.is_some() {
            info!("In main, Key {:?} pressed", key);
        }
        Timer::after_millis(10).await; 
    }             
}

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