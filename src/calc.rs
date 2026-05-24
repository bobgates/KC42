// This module implements the calculation. It takes key presses and returns
// a list of Styles defined by embedded graphics that are used in main to
// update the screen

// use embassy_sync::channel::Channel;
// use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use core::char;
use core::f32;
use defmt::println;
// use embedded_graphics::prelude::*;
// use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::BinaryColor;


use heapless::Vec;
use heapless::String;
// use std::vec::Vec;
use crate::keyboard::KeyName;

use defmt::info;

pub struct Calc {
    // num_buffer: Vec<u8, 20>,  // Holds the numbers while they're being entered
    num_buffer: Vec<u8,64>,
    num_has_point: bool,        // Track if num_buffer has a decimal point in it
    stack: Vec<f64,4>,
    style: MonoTextStyle<'static,BinaryColor>,
    line: String<40>,
}


impl Calc {
    pub unsafe  fn new() -> Calc {
        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        static mut LINE: String<40> = String::new();

        Calc { 
            num_buffer: Vec::<u8,64>::new(),
            num_has_point: false,
            stack: Vec::<f64, 4>::new(),
            style,
            line: String::new(),
        }
    }

    // Empty the num_buffer
    pub fn clear_num(&mut self) {
        self.num_buffer.clear();

    }

    // Called if a number key or the decimal point
    // is pressed
    pub fn process_number(&mut self, key: KeyName){
        if (key as u8) < 10{
            self.num_buffer.push(key as u8).expect("digit must be in the range 0-9 or .");   
            info!("{} pressed", key as u8);
        } else {
            info!("{:?} pressed", key);
        }
        if key == KeyName::DecimalPoint {
            if !self.num_has_point {
                self.num_has_point = true;
                self.num_buffer.push(key as u8).expect("key must be .");  
            } 
        } 
    }

// Proces Enter next


    pub fn input_key<'a>(&mut self, key: Option<KeyName>)->
                        Option<String<40>>{
        if key==Option::None {
            return None;
        }
        let key = key.unwrap();
        self.process_number(key);
        info!("Num buffer: ");
        for n in self.num_buffer.clone(){
            info!("{}", n as u8);
        }
        info!(" \n");
        // info!("Number buffer:({})", self.num_buffer as f32);

        // Some(output)
        None
    }
}

// Need to convert number buffer into an actual number!