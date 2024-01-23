use std::io::{self, stdout};
use std::io::Read;
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};


fn to_ctrl_byte(c:char) -> u8 {
    let byte = c as u8;
    byte & 0b001_1111
}

fn main() {
    enable_raw_mode().expect("fail to enable eaw mode");
    let _stdout = stdout();

    for num in io::stdin().bytes() {

        match num {
        Ok(num) => {
            let c = num as char;

            if c.is_control(){
                println!("(control {}) '{}' \r", num, c);
            } else {
                println!("({}) '{}' \r", num, c);
            }
            if num == to_ctrl_byte('q') {
                break;
            }
        },
        Err(err) => panic!("{}", err)  
        } 
    }
    disable_raw_mode().expect("Failed to disable raw mode");
}
