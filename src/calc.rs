// This module implements the calculation. It takes key presses and returns
// a list of Styles defined by embedded graphics that are used in main to
// update the screen

// use embassy_sync::channel::Channel;
// use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use core::char;
use core::f32;
use core::num;
use cortex_m::peripheral::nvic;
// use core::ops::range;
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

const DOT: u8 = '.' as u8;
const E: u8 =  KeyName::E as u8;
const MINUS: u8 = '-' as u8;
const UNDERSCORE: u8 = '_' as u8;
const COMMA: u8 = ',' as u8;


// Format for the display of numbers. It can be one of those below.
// The numeric parameter is the number of decimal points
enum NumFormat {
    Eng(u8),
    Sci(u8),
    Fix(u8),
}


pub struct Calc {
    // num_buffer: Vec<u8, 20>,  // Holds the numbers while they're being entered
    num_buffer: Vec<u8,64>,
    num_has_point: bool,        // Track if num_buffer has a decimal point in it
    editing: bool,
    num_has_exponent: bool,
    num_is_negative: bool,
    num_format: NumFormat,
    stack: Stack,
    style: MonoTextStyle<'static,BinaryColor>,
    line: String<40>,

}

impl Calc {
    pub unsafe  fn new() -> Calc {
        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        //static mut LINE: String<40> = String::new(); // Line to hold x number for editing

        let mut num_buffer = Vec::<u8,64>::new();
        num_buffer.push('_' as u8).expect("Failed to push '_' into num_buffer in Calc::new()");
        Calc { 
            num_buffer,//: Vec::<u8,64>::new(),    // A text line used for editing and converted to a number
            num_has_point: false,
            num_has_exponent: false,
            num_is_negative: false,
            num_format: NumFormat::Eng(4),
            editing: true,
            stack: Stack::new(),
            style: style,
            line: String::new(),
        }
    }

    pub fn update_stack_display(self){

    }


    // Called if a number key, +-, E key, enter key, backspace or the decimal point
    // is pressed
    pub fn update_numbuffer(&mut self, key: KeyName){
        // info!("{} pressed", key as u8);

        if (key as u8) < 10{
            if self.editing {
                let d = self.num_buffer.pop().expect("Failed to pop from num_buffer");
                if d != '_' as u8 {
                    info!("_ was expected but {} was found",d);   // remove the _ character
                }
                self.num_buffer.push(key as u8).expect("digit must be in the range 0-9 or .");   
                self.num_buffer.push(d as u8).expect("Failed to push _ into num_buffer");

                // info!("{} number", key as u8);
            } else {
                info!("in not editing update_numbuffer ");
                self.num_buffer.clear();
                self.num_buffer.push(key as u8).expect("digit must be in the range 0-9 or .");   
                let _ = self.num_buffer.push('_' as u8);
                self.editing = true;
            }
        } else {
            match key{
                KeyName::DecimalPoint => if !self.num_has_point {
                    info!("Decimal point: pressed");
                    if !self.num_has_point{ 
                        self.num_has_point = true;

                        self.num_buffer.push(key as u8).expect("key must be ."); 
                    } 
                },
                KeyName::PlusMinus => {
                    info!("+/- pressed");
                    self.num_is_negative = !self.num_is_negative;

                }
                KeyName::Back => {
                    info!("back pressed");
                    if self.editing {
                        info!("In editing mode");
                        if self.num_buffer.len()>1{
                            let key = self.num_buffer.pop().unwrap();
                            if key == '.' as u8 {
                                self.num_has_point=false;
                            };
                            if key == 'E' as u8 {
                                info!("in E");
                                self.num_has_exponent=!self.num_has_exponent;
                            }
                        // } else if self.num_buffer.len()==1{
                        //     let _ = self.editing = false;
                        //     let _ = self.num_buffer.pop().unwrap();
                        //     // ********* self.zero_to_numbuffer();
                        //     let _ = self.num_buffer.push(0);
                        //     let _ = self.num_buffer.push(DOT);
                        //     let _ = self.num_buffer.push(0);
                        //     let _ = self.num_buffer.push(0);
                        //     let _ = self.num_buffer.push(0);
                        //     let _ = self.num_buffer.push(KeyName::E as u8);
                        //     let _ = self.num_buffer.push(0);
                        }
                    } else {
                        let _ = self.editing = false;
                        let _ = self.num_buffer.pop().unwrap();
                        // ********* self.zero_to_numbuffer();
                        let _ = self.num_buffer.push(0);
                        let _ = self.num_buffer.push('.' as u8);
                        let _ = self.num_buffer.push(0);
                        let _ = self.num_buffer.push(0);
                        let _ = self.num_buffer.push(0);
                    }
                    
                    // HANDLE NON-EDITING MODE
                    
                    // else {
                    //     info!("Not in editing mode");
                    //     let _ = self.num_buffer.push(0);
                    //     let _ = self.num_buffer.push('.' as u8);
                    //     let _ = self.num_buffer.push(0);
                    //     let _ = self.num_buffer.push(0);
                    //     let _ = self.num_buffer.push(0);
                    // }
                }
                KeyName::E => if !self.num_has_exponent {
                    info!("E pressed");
                   
                    self.num_has_exponent = true;
                    self.num_buffer.push(key as u8).expect("key should be E");
                }
                KeyName::Enter => {
                    info!("Enter pressed");
                    self.stack.push();
                    self.editing = false;
                }
                _ => {},
            }
        }
    }

    // Takes the number buffer and puts it into an f64
    fn to_num(&mut self){

    }

    // Takes an f64 and puts it into display form in self.num_buffer
    fn from_num(&mut self, _num: f64){

    }

    // Takes a key stroke and figures out what to do with it
    pub fn process_key<'a>(&mut self, key: Option<KeyName>)->Option<String<40>>{
        if key==Option::None {  // It gets called every key read loop
            return None;
        } 
        let key: KeyName = key.unwrap(); // Safe because None case is handled above

        // info!("About to call update_numbuffer");
        self.update_numbuffer(key);   
        
        // info!("---------numbuffer starts: {}", self.num_buffer.clone());
        for c in self.num_buffer.clone(){
            info!("{}", c);
        }
        info!("---------");
        // info!("{:?}", self.num_buffer.clone());

        // Create the characters that represent the bottom
        // of the stack. But what if it is empty?
        self.line.clear();
        if self.num_is_negative {
            self.line.push('-').unwrap();
        }
        // Move this into
        if self.num_buffer.len()>0{
            for n in self.num_buffer.clone(){
                let last = self.line.pop();
                match n {
                    0..=9 => if let Some(c) = char::from_digit(n.into(), 10){
                                    self.line.push(c).unwrap();
                                    info!("pushed number {}",c);
                            },    
                   37 => { self.line.push('.').unwrap();
                                    info!("pushed .");      
                            },
                    E => { self.line.push('e').unwrap();
                                    info!("pushed e");      
                            },
                    MINUS => { self.line.push('-').unwrap();        //95 is the ASCII code for underscore, but it is used as a placeholder for the end of the number buffer, so it shouldn't be displayed. The actual minus sign is 45.
                                    info!("pushed -");      
                            },
                    UNDERSCORE =>{ self.line.push('_').unwrap();
                                    info!("pushed _");      
                            },
                    // COMMA=> {self.line.push(',').unwrap();
                    //                 info!("pushed ,")
                    //         }
                    _ => info!("Number buffer contains {} -  couldn't be cloned", n),
                }
                self.line.push(last.unwrap()).unwrap();  // Add the _ back in   
            }
        } else {
            info!("Num_buffer is empty");
        }

        if self.line.len()>0 {
            Some(self.line.clone())
        } else {
            None
        }
    }
}



// Implement ENTER
// Need to convert number buffer into an actual number!
// Still allows edits when it should be in non-edit mode
// NEED to process backspace
// Doesn't backspace - probably needs _ character to be tested and
// used more thoroughly.
