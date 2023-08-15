mod attr;
mod screen;

use std::{
    env,
    io::{self, Read},
};

enum Mode {
    Command,
    Insert,
}

fn main() {
    let mode = Mode::Command;
    let mut buffer = String::new();

    // if file specified, load it into buffer
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        buffer = std::fs::read_to_string(&args[1]).unwrap();
    }

    // set terminal to raw mode
    let mut termios = attr::get_terminal_attr().unwrap();
    attr::make_raw_terminal_attr(&mut termios);
    attr::set_terminal_attr(&termios);

    loop {
        screen::clear();
        screen::draw();

        let stdin = io::stdin();
        let byte = stdin.bytes().next().unwrap().unwrap();
        // print!("byte: {:?}\r\n", byte);

        if byte == 0x03 {
            screen::clear();
            break;
        }
    }
}
