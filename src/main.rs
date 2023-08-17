mod attr;
mod input;
mod screen;

use libc::termios as Termios;
use std::{
    env, fs,
    io::{self, Read},
};

#[derive(Debug)]
pub struct Vector2D {
    x: usize,
    y: usize,
}

pub struct EditorConfig {
    original_termios: Termios,
    dimensions: Vector2D,
}

fn main() {
    let editor_config = EditorConfig {
        original_termios: attr::get_terminal_attr().unwrap(),
        // dimensions: screen::get_dimensions().unwrap(),
        dimensions: Vector2D { x: 0, y: 0 },
    };

    println!("dimensions: {:?}", editor_config.dimensions);

    attr::set_terminal_raw_mode();

    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("") == 1 && buf != [0x03] {
        screen::clear();
        println!("buf: {:?}", buf);
        // let byte = input::read_key();
    }

    // loop {
    //     screen::clear();
    //     let byte = input::read_key();
    //
    //     if byte == 0x03 {
    //         screen::clear();
    //         attr::set_terminal_attr(&editor_config.original_termios);
    //         break;
    //     } else {
    //         println!("{}", byte);
    //     }
    // }
}
