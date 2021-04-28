use std::io::{self, stdout, Read};

use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!(e);
}

fn main() {
    // configure Raw Terminal
    let _stdout = stdout().into_raw_mode().unwrap();

    for bytes in io::stdin().bytes() {

        match bytes {
            Ok(key_bytes) => {
                let key = key_bytes as char;

                if key.is_control() {
                    // Print the bytes
                    println!("{:?} \r", key_bytes);
                } else {
                    // Print the bytes and the char?
                    println!("{:?} ({})\r", key_bytes, key);
                }

                // Ctrl + C or ESC to exit
                if key_bytes == to_ctrl_byte('c') || key_bytes == 27 {
                    break;
                }
            },
            Err(err) => die(err)
        }
        
    }
}
