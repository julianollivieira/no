mod attr;
mod input;
mod screen;

use attr::IntoRawMode;
use std::{
    env, fs,
    io::{self, Read, Write},
};

// use crate::input::async_stdin;

#[derive(Debug)]
pub struct Vector2D {
    x: usize,
    y: usize,
}

pub struct EditorConfig {
    dimensions: Vector2D,
}

fn main() -> io::Result<()> {
    let mut stdin = input::async_stdin().bytes();
    let mut stdout = io::stdout()
        .lock()
        .try_into_raw_mode()
        .expect("failed to enable raw mode for stdout");

    // get terminal dimensions
    screen::move_cursor_to_bottom_right(&mut stdout.output);
    let pos = screen::get_cursor_position(&mut stdin, &mut stdout.output);

    // dbg!(pos);

    loop {
        // screen::clear(&mut stdout.output)?;
        // TODO: print dimensions in dbg

        let b = stdin.next();
        if let Some(Ok(b)) = b {
            if b == 0x03 {
                screen::clear(&mut stdout.output)?;
                // attr::set_terminal_attr(&editor_config.original_termios);
                break Ok(());
            } else {
                // buf.push(b);
                // write!(stdout.output, "b: {:?}", b)?;
                // stdout.output.flush().unwrap();
            }
        }
    }

    // let editor_config = EditorConfig {
    //     original_termios: attr::get_terminal_attr().unwrap(),
    //     // dimensions: screen::get_dimensions().unwrap(),
    //     dimensions: Vector2D { x: 0, y: 0 },
    // };

    // println!("dimensions: {:?}", editor_config.dimensions);
    // attr::set_terminal_raw_mode();

    // let mut buf = [0; 1];
    // while async_stdin().read(&mut buf).unwrap() == 1 && buf != [0x03] {
    //     screen::clear();
    //     println!("buf: {:?}", buf);
    // }

    // let mut buf = [0; 1];
    // while io::stdin().read(&mut buf).expect("") == 1 && buf != [0x03] {
    //     screen::clear();
    //     println!("buf: {:?}", buf);
    //     // let byte = input::read_key();
    // }

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
