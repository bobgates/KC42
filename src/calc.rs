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
use heapless::string::StringInner;
use heapless::Vec as VecStorage;
use heapless::format;
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

const DP: u8 = '.' as u8;           // 46 decimal
const E: u8 =  KeyName::E as u8;    // 69 decimal
const MINUS: u8 = '-' as u8;        // 45 decimal
const UNDERSCORE: u8 = '_' as u8;   // 95
const BACK: u8 = KeyName::Back as u8;  // 66 decimal
const PLUSMINUS: u8 = KeyName::PlusMinus as u8;  // 30 decimal


// Format for the display of numbers. It can be one of those below.
// The numeric parameter is the number of decimal points
enum NumFormat {
     Eng(u8),
//     Sci(u8),
//     Fix(u8),
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
    // line: String<40>,

}

impl Calc {

    // Starts with just _ in the num_buffer,    
    pub unsafe fn new() -> Calc {

        info!("In Calc::new");

        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        //static mut LINE: String<40> = String::new(); // Line to hold x number for editing

        let mut num_buffer = Vec::<u8,64>::new();
        num_buffer.push('_' as u8).expect("Failed to push '_' into num_buffer in Calc::new()");
        Calc { 
            num_buffer,    // A text line used for editing and converted to a number
            num_has_point: false,
            num_has_exponent: false,
            num_is_negative: false,
            num_format: NumFormat::Eng(4),
            editing: true,
            stack: Stack::new(),
            style: style,
            // line: String::new(),
        }
    }


    // Takes a key stroke and figures out what to do with it
    pub fn process_key<'a>(&mut self, key: Option<KeyName>, num_buffer:  &Vec<u8, 64>)->Option<String<64>>{//<Vec::<u8,64>>{
        if key==Option::None {  // It gets called every key read loop
            // return None;
        } 
        let key: KeyName = key.unwrap(); // Safe because None case is handled above
        info!("{}",key as u8);

        let mut num_buffer_str: String<64> = String::new(); //= Vec::<char, 64>::new();


        match key as u8 {
            n @ 0..=9 => {
                        if !num_buffer.contains(&('_' as u8)) { // We're starting a new number, so clear the buffer and put the _ back in
                            info!("New edit triggered by number");
                            self.num_buffer.clear();
                            self.num_buffer.push(n).expect("failed to push digit into num_buffer in process_key()");  // Add the digit to the buffer
                            self.num_buffer.push('_' as u8).expect("Failed to push '_' into num_buffer in process_key()");  // Put the _ back in so it shows on the display
                        } else {    // We're in the middle of editing a number, so just add the digit to the buffer
                            self.num_buffer.insert(self.num_buffer.len()-1, n).expect("Failed to insert digit into num_buffer in process_key()");  // Insert the digit before the _ character
                        }
                    }
            DP  => { self.num_buffer.push(b'.').unwrap();
                            info!("pushed . as {}", b'.');      
                    }
            E => {info!("pushed e"); 
                 self.num_buffer.push(b'e').unwrap();
                            info!("pushed e");      
                    }
            MINUS => { self.num_buffer.push(b'-').unwrap();        //95 is the ASCII code for underscore, but it is used as a placeholder for the end of the number buffer, so it shouldn't be displayed. The actual minus sign is 45.
                            info!("pushed -");      
                    }
            UNDERSCORE =>{ self.num_buffer.push(b'_').unwrap();
                            info!("pushed _");      
                    }
            // _ => info!("Number buffer contains {} -  couldn't be cloned", n),
            BACK => {
                info!("back pressed");
                // if self.num_buffer.len()>1{
                //     let last_key = self.num_buffer.pop().unwrap();
                //     if key == BACK && self.num_buffer.len()>1 {  // If the back key is pressed and there's more than one character in the buffer, pop the last character
                //         let k = self.num_buffer.pop();

                //     if key == '.' as u8 {
                //         self.num_has_point=false;
                //     };
                //     if key == 'e' as u8 {
                //         self.num_has_exponent=false;
                //     }
                // }
            },
            _ => info!("not yet implemented for {}", key as u8),

        };
                // let t = last.expect("Failed to pop from line");
info!("first num_buffer contains {} after processing number key", self.num_buffer.as_slice());

        // Transfer self.num_buffer into a Vec<u8,64> for display. This is a bit convoluted but it allows us to keep the num_buffer as a Vec<u8,64> for editing and then convert it to a String for display and back to a Vec<u8,64> to return it.  

        for c in self.num_buffer.iter() {
            if *c<=9{
                let d:u32 = (*c).into();
                num_buffer_str.push(char::from_digit(d, 10).expect("Failed to push number into num_buffer_str in process_key()")).unwrap();  // Convert the u8 in num_buffer to a char and push it into num_buffer_str for display
            } else {
                // num_buffer_str.push('B').expect("Error converting in process_key()");//char::from(*c)).expect("Failed to push character into num_buffer_str in process_key()");  // Convert the u8 in num_buffer to a char and push it into num_buffer_str for display
                    let x = match *c as u8 {
                        DP => '.',
                        E => 'e',
                        MINUS => '-',
                        PLUSMINUS => '±',
                        UNDERSCORE => '_',
                        _ => char::from(*c)
                    };
                num_buffer_str.push(x).expect("Failed to push character into num_buffer_str in process_key()");
            }

            info!("num_buffer_str contains {} in process_key()", num_buffer_str.as_str());

            //     let ch = *c as u32;
            //     num_buffer_str.push(char::from_u32(ch).expect("Error converting char in process_key")).expect("Failed to push character into num_buffer_str in process_key()");  // Convert the u8 in num_buffer to a char and push it into num_buffer_str for display;
            // }
            // for (count, i) in num_buffer_str.chars().enumerate() {
            //     info!("B{}{}", count, i);
            //     let d: u32 = i.into();
            //     let e:char = char::from_u32(d).expect("Error converting char in process_key() for display");
            //     info!("{}: {}", count, e);
            //     // info!("num_buffer contains {}-{} in process_key()", i as u8, char::from_digit(i as u32, 10).unwrap());
            // }
            // // num_buffer_str 
            // .push(*c as char).expect("Failed 
            // to push character into num_buffer_str in process_key()");  
            // Convert the u8 in num_buffer to a char and push it into num_buffer_str for display
        }   
        Some(num_buffer_str)  
    }



}



// Implement ENTER
// Need to convert number buffer into an actual number!
// Still allows edits when it should be in non-edit mode
// NEED to process backspace
// Doesn't backspace - probably needs _ character to be tested and
// used more thoroughly.
