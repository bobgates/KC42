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

pub struct Stack{
    x: f64,
    y: f64,
    z: f64,
    a: f64,
}

impl Stack {
    pub fn new()-> Stack{
        Stack { x: 0.0, y: 0.0, z: 0.0, a: 0.0 }
    }
    pub fn push(&mut self) {
        self.a = self.z;
        self.z = self.y;
        self.y = self.x;
        // Leaves x in y and in x
    }
    pub fn pop(&mut self) {
        self.x = self.y;
        self.z = self.z;
        self.z = self.a;
        // Leaves a in a and z
    }
}



pub struct Calc {
    // num_buffer: Vec<u8, 20>,  // Holds the numbers while they're being entered
    num_buffer: Vec<u8,64>,
    num_has_point: bool,        // Track if num_buffer has a decimal point in it
    editing: bool,
    num_has_exponent: bool,
    num_is_negative: bool,
    stack: Stack,
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
            num_has_exponent: false,
            num_is_negative: false,
            editing: true,
            stack: Stack::new(),
            style: style,
            line: String::new(),
        }
    }

    // Empty the num_buffer
    pub fn clear_num(&mut self) {
        self.num_buffer.clear();
    }

    pub fn update_stack_display(&mut self)->(f64, f64, f64){
        (self.stack.y, self.stack.z,self.stack.a)
    }

    // Called if a number key, +-, E key or the decimal point
    // is pressed
    pub fn process_number(&mut self, key: KeyName){
        // info!("{} pressed", key as u8);

        if (key as u8) < 10{
            self.num_buffer.push(key as u8).expect("digit must be in the range 0-9 or .");   
            info!("{} number", key as u8);
        } else {
            match key{
                KeyName::DecimalPoint=> if !self.num_has_point {
                    info!("Decimal point: pressed");
                    if !self.num_has_exponent{ 
                        self.num_has_point = true;
                        self.num_buffer.push(key as u8).expect("key must be ."); 
                    } 
                },
                KeyName::PlusMinus => {
                    info!("+/- pressed");
                    self.num_is_negative = !self.num_is_negative;

                }
                KeyName::E => if !self.num_has_exponent {
                    info!("E pressed");
                   
                    self.num_has_exponent = true;
                    self.num_buffer.push(key as u8).expect("key should be E");
                }
                KeyName::Enter => {
                    info!("Enter pressed");
                    self.stack.push();
                }
                _ => {},
            }
        }
    }


    pub fn input_key<'a>(&mut self, key: Option<KeyName>)->Option<String<40>>{
        if key==Option::None {
            return None;
        }
        let key = key.unwrap();
        self.process_number(key);

        self.line.clear();
        if self.num_is_negative {
            self.line.push('-').unwrap();
        }
        for n in self.num_buffer.clone(){
            if n == KeyName::DecimalPoint as u8{
                self.line.push('.').unwrap();
            } else if n == KeyName::E as u8 {
                self.line.push('E').unwrap();
            } else {
                let c = char::from_digit(n.into(), 10).unwrap();
                self.line.push(c).unwrap();
            }

        }
        if self.line.len()>0 {
            // info!("{}", self);
            Some(self.line.clone())
        } else {
            None
        }
    }
}

// Need to convert number buffer into an actual number!