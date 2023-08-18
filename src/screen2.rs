use crate::Vector2D;
use libc::{STDOUT_FILENO, TIOCGWINSZ};
use std::{
    env,
    error::Error,
    fs,
    io::{self, Read, Write},
};

/// Clears the screen and moves the cursor to the top left corner.
pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/// Moves the cursor to the top left corner.
pub fn move_cursor_to_bottom_right() {
    print!("{esc}[999C{esc}[999B", esc = 27 as char);
}

/// Gets the dimensions of the terminal.
pub fn get_dimensions() -> Result<Vector2D, Box<dyn Error>> {
    // let winsize = unsafe { libc::ioctl(STDOUT_FILENO, TIOCGWINSZ) };

    // TODO: uncomment and implement reading winsize from u32??
    // if winsize != -1 || winsize != 0 {
    //     return Ok(Vector2D { x: 0, y: 0 });
    // }

    move_cursor_to_bottom_right();
    Ok(get_cursor_position())
}

pub fn get_cursor_position() -> Vector2D {
    let mut buf = [0_u8; 12];
    print!("{esc}[6n", esc = 27 as char);
    io::stdout().flush().unwrap();
    io::stdin().read(&mut buf).unwrap();
    let buf = String::from_utf8_lossy(&buf);
    let mut iter = buf[2..].split(';');
    let row = iter.next().unwrap().parse::<u16>().unwrap();
    let col = iter.next().unwrap().parse::<u16>().unwrap();

    // println!("aa");

    Vector2D {
        x: col.into(),
        y: row.into(),
    }
}

// Gets the position of the cursor.
// pub fn get_cursor_position() -> Vector2D {
//     // let mut buf = [0 as char; 32];
//     let mut buf = Vec::new();
//     let mut i = 0;
//
//     print!("{esc}[6n", esc = 27 as char);
//
//     while (i < buf.len()) {
//         buf[i] = io::stdin().bytes().next().unwrap().unwrap() as char;
//         print!("{c}", c = buf[i]);
//
//         if buf[i] == 'R' {
//             break;
//         }
//
//         i += 1;
//     }
//
//     dbg!(buf);
//
//     Vector2D { x: 0, y: 0 }

// print!("{esc}[6n", esc = 27 as char);
// print!("\r\n");
//
// let mut buf = [0_u8; 1];
// io::stdin().read(&mut buf).unwrap();

// if buf[0] as char == '[' {
//     print!("{c}\r\n", c = buf[0] as char);
//     // print!("{esc}[6n", esc = 27 as char);
// } else {
//     print!("{c} ({d})\r\n", c = buf[0] as char, d = buf[0]);
// }

// return Vector2D { x: 0, y: 0 }; //

// print!("{esc}[6n", esc = 27 as char);
// io::stdout().flush().unwrap();
//
// let mut buf = [0_u8; 12];
// io::stdin().read(&mut buf).unwrap();
// let buf = String::from_utf8_lossy(&buf);
//
// let mut iter = buf[2..].split(';');
// let y = iter.next().unwrap().parse::<u16>().unwrap();
// let x = iter.next().unwrap().parse::<u16>().unwrap();
//
// println!("x: {}, y: {}", x, y);
//
// Vector2D {
//     x: x.into(),
//     y: y.into(),
// }

// let mut buf = [0_u8; 12];
// print!("\x1B[6n");
// io::stdout().flush().unwrap();
// io::stdin().read(&mut buf).unwrap();
// let buf = String::from_utf8_lossy(&buf);
// let mut iter = buf[2..].split(';');
// let row = iter.next().unwrap().parse::<u16>().unwrap();
// let col = iter.next().unwrap().parse::<u16>().unwrap();
// (row, col)
// }

// pub fn draw(config: &EditorConfig) {
//     for _ in 0..config.dimensions.height {
//         print!("~\r\n");
//     }
// }
//
// if file specified, load it into buffer
// let args: Vec<String> = env::args().collect();
// if args.len() > 1 {
//     buffer = fs::read_to_string(&args[1]).unwrap();
// }
