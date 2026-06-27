#![allow(unused_imports)]


use defmt::*;

use heapless::String;

#[derive( Clone)]
pub struct Stack{
    x: f64,
    y: f64,
    z: f64,
    t: f64,
    changed: bool,
    x_str: String<64>,
    y_str: String<64>,
    z_str: String<64>,
    t_str: String<64>,
}

impl Stack {
    pub fn new()-> Stack{
        Stack { x: 0.0, 
            y: 0.0, 
            z: 0.0, 
            t: 0.0, 
            changed: false,
            x_str: String::try_from("0.000").unwrap(),
            y_str: String::try_from("0.000").unwrap(),
            z_str: String::try_from("0.000").unwrap(),
            t_str: String::try_from("0.000").unwrap(),
        }
    }
    pub fn push(&mut self, x: f64) {
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
        self.x = x;
        self.changed = true;
        // Leaves x in y and in x
    }

    // Pops and returns bottom, x, value
    pub fn pop(&mut self)-> f64 {
        let temp = self.x;
        self.x = self.y;
        self.y = self.z;
        self.z = self.t;
        self.changed = true;
        // Leaves a in a and in z
        temp
    }
    pub fn set_changed(&mut self) {
        self.changed = true;
    }
    pub fn changed(&mut self)->bool{
        self.changed
    }
    
    pub fn fetch_values(&mut self) -> (f64, f64, f64, f64){
        (self.x, self.y, self.z, self.t)
    }

    pub fn fetch_strs(&mut self) -> (&str, &str, &str, &str){
        (&self.x_str, &self.y_str, &self.z_str, &self.t_str)
    }

    pub fn fill_y_to_t(&mut self, y:f64, z: f64, t:f64){
        self.y = y;
        self.z = z;
        self.t = t;
    }

    pub fn swapxy(&mut self){
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }


    pub fn swapx_with_new_y(&mut self, new_y: f64){
        self.x = self.y;
        self.y = new_y;
    }

    pub fn debug(&mut self) {
        info!("x:{}, y:{}, z:{}, t:{}", self.x, self.y, self.z, self.t);
    }


    pub fn get_x(&mut self)->f64{
        return self.x;
    }
    
    pub fn get_y(&mut self)->f64{
        return self.y;
    }

}
