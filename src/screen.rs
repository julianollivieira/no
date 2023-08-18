use std::io::{self, Bytes, Read, StdoutLock, Write};

use crate::{attr::RawTerminal, input::AsyncReader, Vector2D};

const ESC: char = 27 as char;

/// Clears the screen.
pub fn clear(w: &mut impl io::Write) -> io::Result<()> {
    write!(w, "{ESC}[2J{ESC}[1;1H")
}

/// Moves the cursor to the bottom left corner.
pub fn move_cursor_to_bottom_right(w: &mut impl io::Write) -> io::Result<()> {
    write!(w, "{ESC}[999C{ESC}[999B")
}

/// Gets the dimensions of the terminal.
pub fn get_cursor_position(stdin: &mut Bytes<AsyncReader>, stdout: &mut StdoutLock) -> Vector2D {
    let mut buf: Vec<char> = Vec::new();
    let mut i = 0;

    write!(stdout, "{ESC}[6n").unwrap();
    stdout.flush().unwrap();

    // while i < buf.len() - 1 {
    while i < buf.len() {
        let b = stdin.next();
        if let Some(Ok(b)) = b {
            buf.push(b as char);
        } else {
            break;
        }

        if buf[i] == 'R' {
            break;
        }

        i += 1;
    }

    dbg!(buf);

    // print!("{ESC}[6n");
    // stdout.flush().unwrap();

    // stdout.flush().unwrap();

    // stdin.read(&mut buf).unwrap();
    // while
    // stdin.

    // let mut i = 0;
    // while let Some(b) = stdin.next() {
    //     buf[i] = b.unwrap();
    //
    //     i += 1;
    //
    //     if i == buf.len() + 1 {
    //         break;
    //     }
    // }
    //
    // dbg!(buf);
    //
    // let buf = String::from_utf8_lossy(&buf);
    // let mut iter = buf[2..].split(';');
    // let row = iter.next().unwrap().parse::<u16>().unwrap();
    // let col = iter.next().unwrap().parse::<u16>().unwrap();
    //
    // // println!("aa");
    //
    // Vector2D {
    //     x: col.into(),
    //     y: row.into(),
    // }

    Vector2D { x: 0, y: 0 }
}
