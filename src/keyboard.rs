use defmt::{/*error, info, println, warn,*/ Format};

// use embassy_rp as hal;
// use embassy_executor::Spawner;
// use embassy_rp::block::ImageDef;
use embassy_time::Timer;
// use rp235x_hal::Timer;
// use embassy_rp::Peripherals;
// use embassy_rp::gpio::{Pull};//, Level, Input, Output};
use embassy_rp::{gpio::{Input, Output}};//, multicore::current_core};

// use std::ops::{Index, IndexMut};

#[derive(Debug, Format, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// pub enum KeyName{//     Fn1=1, Fn2, Fn3, Fn4, Fn5, Fn6,//     SigmaPlus,//     Invert,//     Sqrt,
//     Log,//     Ln,//     Xeq,//     Sto,//     Rcl,//     RollDown,//     Sin,
//     Cos,//     Tan,//     Enter,//     XswapY,//     PlusMinus,//     E,//     Back,
//     Up,//     Down,//     Orange,//     Exit,//     DecimalPoint,//     RunStop,
//     Plus,//     Minus,//     Divide,//     Multiply,//     Number(u8),//     Error,// }
pub enum KeyName{
    Number0=0, // Setting the first to a number starts an auto-numbering system
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    Fn1,
    Fn2, 
    Fn3, 
    Fn4, 
    Fn5, 
    Fn6,
    SigmaPlus,
    Invert,
    Sqrt,
    Log,
    Ln,
    Xeq,
    Sto,
    Rcl,
    RollDown,
    Sin,
    Cos,
    Tan,
    Enter,
    XswapY,
    PlusMinus,
    E,
    Back,
    Up,
    Down,
    Orange,
    Exit,
    DecimalPoint,
    RunStop,
    Plus,
    Minus,
    Divide,
    Multiply,

    Error,
}
// #[derive(Debug, Clone, Copy)]
static ROW_COL_MAP: [[KeyName; 6]; 8] = [
    [KeyName::Exit, KeyName::Number0, KeyName::DecimalPoint, KeyName::Error,  KeyName::RunStop, KeyName::Plus],
    [KeyName::Orange, KeyName::Number1, KeyName::Number2, KeyName::Error,  KeyName::Number3, KeyName::Minus],
    [KeyName::Down, KeyName::Number4, KeyName::Number5, KeyName::Error,  KeyName::Number6,KeyName::Multiply],
    [KeyName::Up, KeyName::Number7, KeyName::Number8, KeyName::Error, KeyName::Number9, KeyName::Divide],
    [KeyName::Enter, KeyName::Enter, KeyName::XswapY, KeyName::PlusMinus, KeyName::E, KeyName::Back],
    [KeyName::Sto, KeyName::Rcl, KeyName::RollDown, KeyName::Sin, KeyName::Cos, KeyName::Tan],
    [KeyName::SigmaPlus, KeyName::Invert, KeyName::Sqrt, KeyName::Log, KeyName::Ln, KeyName::Xeq],
    [KeyName::Fn1, KeyName::Fn2, KeyName::Fn3, KeyName::Fn4, KeyName::Fn5, KeyName::Fn6]
];

struct KeyCount {

}

pub struct Keyboard{

    rows: [Input<'static>; 8],
    cols: [Output<'static>; 6],
    current_key: Option<KeyName>,
}

impl Keyboard {
    pub fn new( row_pins: [Input<'static>; 8], col_pins: [Output<'static>; 6], ) -> Self {
        Self {
            rows: row_pins,
            cols: col_pins,
            current_key: None,
        }
    }

    // Scans the hardware and returns a key, mapped as defined above
    // if one has been pressed, else None
    pub async fn scan(&mut self) ->  Option<KeyName> {//Option<u8> {

        let mut down_count = 0;
        let mut n_row = 0;
        let mut n_col = 0;
        
        for (nc, col) in self.cols.iter_mut().enumerate() {
            Timer:: after_millis(3).await;   

            col.set_high();
            for (nr, row) in self.rows.iter().enumerate()  {
                    if row.is_high() {
                    down_count += 1;
                    // n_true_rows += 1;
                    // info!("{} rows true", n_true_rows); 
                    n_row = nr;
                    n_col = nc;
                }        
            }
            col.set_low();
        }
        if down_count == 0 {
            self.current_key = None;
            return None;    
        }

        if down_count == 1 {                            // Only one key is pressed, so we can identify it
            // info!("Key {:?} pressed", ROW_COL_MAP[n_row][n_col]);
            let key = Some( ROW_COL_MAP[n_row][n_col]);
            if self.current_key == key {
                // info!("self.current_key = key -> returning None");
                    return None;
            } else {
                self.current_key = key;
                return match key{
                    None => None,
                    Some(key)=> Some(key)
                }
            }
        }
        None
    }

}




#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"rust_button_first"),
    embassy_rp::binary_info::rp_program_description!(
        c"your program description"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
