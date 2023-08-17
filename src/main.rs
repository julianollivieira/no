mod attr;
mod screen;

use libc::termios as Termios;
use std::{
    env, fs,
    io::{self, Read},
};

pub struct Dimensions {
    width: usize,
    height: usize,
}

pub struct EditorConfig {
    original_termios: Termios,
    dimensions: Dimensions,
}

fn main() {
    let editor_config = EditorConfig {
        original_termios: attr::get_terminal_attr().unwrap(),
        dimensions: screen::get_dimensions().unwrap(),
    };

    let mut buffer = String::new();

    // if file specified, load it into buffer
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        buffer = fs::read_to_string(&args[1]).unwrap();
    }

    attr::set_terminal_raw_mode();
    // screen::get_dimensions();

    loop {
        screen::clear();
        screen::draw(&editor_config);

        let stdin = io::stdin();
        let byte = stdin.bytes().next().unwrap().unwrap();
        // print!("byte: {:?}\r\n", byte);

        if byte == 0x03 {
            screen::clear();
            attr::set_terminal_attr(&editor_config.original_termios);
            break;
        }
    }
}
