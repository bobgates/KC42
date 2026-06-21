
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
    pub fn push(&mut self, x: f64) {
        self.t = self.z;
        self.z = self.y;
        self.y = self.x;
        self.x = x;
        // self.x = entry;   /
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

    pub fn swapxy(&mut self){
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }


    pub fn swapx_with_new_y(&mut self, new_y: f64){
        self.x = self.y;
        self.y = new_y;
    }


    pub fn get_x(&mut self)->f64{
        return self.x;
    }
    
    pub fn get_y(&mut self)->f64{
        return self.y;
    }

    // pub fn fetch_strs(&mut self) -> (Vec<u8,64>, &str, &str){

    //     let y_str: Vec<u8,64> = number_to_string(self.y).unwrap().clone();
    //     // let y = string_to_number(y_str);

    // // let y_str = format!("{:e}", self.y).expect("failed to convert number_to_string ");
    // // let r = y_str.into_bytes();
   
        

    //     (y_str, "0.0", "0.0")
    // }



    // pub fn print(&mut self) {
    //     info!("  Y: {}   Z: {}   T: {}", self.y, self.z, self.t);
    // }

}
