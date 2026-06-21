// This module implements the calculation. It takes key presses and returns
// a list of Styles defined by embedded graphics that are used in main to
// update the screen

// use embassy_sync::channel::Channel;
// use embassy_sync::blocking_mutex::raw::NoopRawMutex;

use core::char;
use core::f64;
use core::num;
// use core::num;
// use cortex_m::peripheral::nvic;
// use core::ops::range;
// use defmt::println; 
// use embedded_graphics::prelude::*;
// use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::BinaryColor;


use heapless::Vec;
use heapless::String;
// use heapless::string::StringInner;
// use heapless::Vec as VecStorage;
use heapless::format;
use heapless::string;
// use std::vec::Vec;
use crate::keyboard::KeyName;
use crate::keyboard::KeyName::DecimalPoint;

use defmt::info;

#[derive(Copy, Clone)]
pub struct Stack{
    x: f64,
    y: f64,
    z: f64,
    t: f64,
    changed: bool,
}

impl Stack {
    pub fn new()-> Stack{
        Stack { x: 0.0, y: 0.0, z: 0.0, t: 0.0, changed: false}
    }
    pub fn push(&mut self) {
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
        self.changed = true;
        // Leaves x in y and in x
    }
    pub fn pop(&mut self) {
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        self.changed = true;
        // Leaves a in a and in z
    }
    pub fn set_changed(&mut self) {
        self.changed = true;
    }
    pub fn changed(&mut self)->bool{
        self.changed
    }
    
    pub fn fetch_values(&mut self) -> (f64, f64, f64){
        (self.y, self.z, self.t)
    }

    pub fn fetch_strs(&mut self) -> (Vec<u8,64>, &str, &str){

        let y_str: Vec<u8,64> = number_to_string(self.y).unwrap().clone();
        // let y = string_to_number(y_str);

    // let y_str = format!("{:e}", self.y).expect("failed to convert number_to_string ");
    // let r = y_str.into_bytes();
   
        

        (y_str, "0.0", "0.0")
    }



    pub fn print(&mut self) {
        info!("  Y: {}   Z: {}   T: {}", self.y, self.z, self.t);
    }

}

const DP: u8 = KeyName::DecimalPoint as u8;           // 46 decimal
const E: u8 =  KeyName::E as u8;    // 69 decimal
// const MINUS: u8 = '-' as u8;        // 45 decimal
const UNDERSCORE: u8 = '_' as u8;   // 95
const BACK: u8 = KeyName::Back as u8;  // 66 decimal
const PLUSMINUS: u8 = KeyName::PlusMinus as u8;  // 30 decimal
const ENTER: u8 =KeyName::Enter as u8;


// Format for the display of numbers. It can be one of those below.
// The numeric parameter is the number of decimal points
enum NumFormat {
   //  Eng(u8),
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
    // num_format: NumFormat,
    pub stack: Stack,
    stack_changed: bool,
    style: MonoTextStyle<'static,BinaryColor>,
    // line: String<40>,

}
 // Converts the string in self.num_buffer into an f64
pub fn string_to_number(mut s: Vec<u8, 64>)->f64{ 

    // Remove _ if it is there:
    let last = s.pop().unwrap();
    if last != 95 {
        s.push(last).unwrap();
    }

    let s = str::from_utf8(&s).unwrap();//.try_into().expect("internal error string_to_number")).expect("Failure to convert in string to number");
    let t = s;//.expect("failed");

    let result: f64 = t.parse().expect("failure to convert in string_to_number");
    result
}

// Takes the entry_buffer, which is a Vec<u8> and converts it to a string that the graphics can use
pub fn convert_to_string(entry_buffer: Vec<u8, 64>)->Option<String< 64>>{
    
    let mut num_buffer_str: String<64> = String::new(); //= Vec::<char, 64>::new();

    for c in entry_buffer.iter() {
            if *c<=9{
                let d:u32 = (*c).into();
                num_buffer_str.push(char::from_digit(d, 10).expect("Failed to push number into num_buffer_str in process_key()")).unwrap();  // Convert the u8 in num_buffer to a char and push it into num_buffer_str for display
            } else {
                    let x = match *c  {
                        // 95 => '_',
                        // DP => '.',
                        // E => 'e',
                        // MINUS => 'A',
                        PLUSMINUS  => 'B',
                        UNDERSCORE => '_',
                        _ => char::from(*c)
                    };
                num_buffer_str.push(x).expect("Failed to push character into num_buffer_str in process_key()");
            }
        } 
        Some(num_buffer_str)
}

    // Converts a number into what fits into num_buffer
pub fn number_to_string(number: f64)->Option<Vec<u8,64>>{
    let temp_str = format!("{:e}", number).expect("failed to convert number_to_string ");
    let r = temp_str.into_bytes();
    Some(r)
}

impl Calc {
    // At power up:
    // Starts with just _ in the num_buffer,    
    // which also means it is in editing mode.

    pub unsafe fn new() -> Calc {

        // info!("In Calc::new");

        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
        //static mut LINE: String<40> = String::new(); // Line to hold x number for editing

        let mut num_buffer = Vec::<u8,64>::new();
        num_buffer.push('_' as u8).expect("Failed to push '_' into num_buffer in Calc::new()");


        Calc { 
            num_buffer,    // A text line used for editing and converted to a number
            num_has_point: false,
            num_has_exponent: false,
            num_is_negative: false,
            // num_format: NumFormat::Eng(4),
            editing: true,
            stack: Stack::new(),
            stack_changed:false,
            style: style,
            // line: String::new(),
        }
    }

    pub fn stack_changed(self)->bool{
        self.stack_changed
    }

    pub fn fetch_stack(self)-> (f64,f64,f64){
        (self.stack.y, self.stack.z, self.stack.t, )
    }

    // pub fn fetch_stack_str(self, entry_buffer:  Vec<u8,64>)-> String<64> {
    //     let number = string_to_number(entry_buffer.clone());

    //     // number_to_string(number).expect("Failed to convert in process_key")
    // }


                    //     let lc = string_to_number(entry_buffer.clone());

                    // // check if we're in editing mode -last char is '_'. 
                    // // If so, remove '_'
                    // let last = entry_buffer.pop().unwrap();
                    // info!("last: {}", last);
                    // if last != '_' as u8 {
                    //     entry_buffer.push(last);
                    // }

                    // self.stack.x = string_to_number(entry_buffer.clone());

    // Takes a key stroke and figures out what to do with it
    pub fn process_key<'a>(&mut self, key: Option<KeyName>)->Option<String::<64>>{


        let key = match key{
            None => return None,
            Some(k) => k
        };

        let mut entry_buffer=self.num_buffer.clone();       

        match key as u8 {
            n @ 0..=9 => {
                        if !entry_buffer.contains(&('_' as u8)) { // We're starting a new number, so clear the buffer and put the _ back in
                            // info!("Starting new number");
                            entry_buffer.clear();
                            entry_buffer.push(n+48).expect("failed to push digit into entry_buffer in process_key()");  // Add the digit to the buffer
                            entry_buffer.push('_' as u8).expect("Failed to push '_' into entry_buffer in process_key()");  // Put the _ back in so it shows on the display
                            self.editing = true;
                        } else {    
                            // info!("pushing number in// We're in the middle of editing a number, so just add the digit to the buffer
                            entry_buffer.insert(entry_buffer.len()-1, n+48).expect("Failed to insert digit into entry_buffer in process_key()");  // Insert the digit before the _ character
                            // self.editing = false;
                        }
                    }
            DP => { if !entry_buffer.contains(&('.' as u8)){
                    info!("Editing: {}", if  self.editing {"True"} else {"False"});
                        if self.editing {
                            entry_buffer.insert(entry_buffer.len()-1, '.' as u8).expect("Failed to insert decimal point");
                        } else {
                            self.editing = true;
                            entry_buffer.clear();
                            entry_buffer.push(0).unwrap();
                            entry_buffer.push('.' as u8).unwrap();
                            entry_buffer.push('_' as u8).unwrap();                    }
                        } else {
                            info!("Found ")
                        }
                    }
            ENTER => { self.editing = false;
                    // info!("ENTER: numbuffer: {}", entry_buffer.as_slice());

                    let lc = string_to_number(entry_buffer.clone());

                    // check if we're in editing mode -last char is '_'. 
                    // If so, remove '_'
                    let last = entry_buffer.pop().unwrap();
                    info!("last: {}", last);
                    if last != '_' as u8 {
                        entry_buffer.push(last);
                    }

                    self.stack.x = string_to_number(entry_buffer.clone());
                    self.stack.push();
                    self.stack.set_changed();
                    entry_buffer = number_to_string(self.stack.x).expect("Failed to convert in process_key"); // Takes stack.x and formats it for display
                }
            E => {//info!("pushed e"); 
                    if !entry_buffer.contains(&('e' as u8)){
                        entry_buffer.insert(entry_buffer.len()-1, 'e' as u8).expect("Failed to insert E into entry_buffer in process_key()");     
                    }   
                }
            UNDERSCORE =>{ entry_buffer.push(b'_').unwrap();
                            // info!("pushed _");      
                    }
            PLUSMINUS =>{ if entry_buffer.contains(&('e' as u8)){
                            for (p,i) in entry_buffer.clone().into_iter().enumerate(){
                                if i == 'e' as u8 {
                                    let c = entry_buffer[p];
                                    info!("Found '{}' as position p: {}", c as char, p);
                                    if entry_buffer.len()>p+1{ 
                                        // let letter = entry_buffer[p+1]; 
                                        if entry_buffer[p+1] == b'-'{
                                            entry_buffer.remove(p+1);
    
                                        } else {
                                            entry_buffer.insert(p+1, b'-').unwrap();
                                         }
                                    }
                                    break;
                                }
                            }
                    } else {
                        if entry_buffer[0]==b'-'{
                                entry_buffer.remove(0);
                                info!("removed minus");
                            } else {
                                entry_buffer.insert(0, b'-').unwrap(); 
                                info!("added minus");    
                            }
                        }                                   
                    }
            // _ => info!("Number buffer contains {} -  couldn't be cloned", n),
            BACK => {
                info!("back pressed");
                if entry_buffer.len()>1{
                    let last_key = entry_buffer.pop().unwrap();
                    info!("* last key: {}", &last_key);
                    if last_key == UNDERSCORE {
                        if entry_buffer.len()>1 {  // If the back key is pressed and there's more than one character in the buffer, pop the last character
                            let k = entry_buffer.pop();
                            info!("Popped second thing - last key is {}", &last_key);
                            entry_buffer.push(last_key).expect("Failed pushing key in process_key:BACK");
                        } else {
                            entry_buffer.pop();
                            for c in "0.000".as_bytes(){
                                entry_buffer.push(*c).unwrap();
                            }
                            self.editing = false;
                        }  
                        
                    }
                    if last_key == KeyName::DecimalPoint as u8 {
                        self.num_has_point=false;
                    };
                    if last_key == KeyName::E as u8{
                        self.num_has_exponent=false;
                    }
                } else { // only one thing left in buffer

                }
                },
            _ => info!("not yet implemented for {}", key as u8),

        };

        // This is a bit convoluted but it allows us to keep the num_buffer as a Vec<u8,64> 
        // for editing and then convert it to a String for display and back to a Vec<u8,64> to return it.  

        self.num_buffer = entry_buffer.clone(); // Vec<u8>

        convert_to_string(entry_buffer) // String for display

    }    
     


}



// Implement ENTER
// Need to convert number buffer into an actual number!
// Still allows edits when it should be in non-edit mode
// NEED to process backspace
// Doesn't backspace - probably needs _ character to be tested and
// used more thoroughly.
