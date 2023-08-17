use crate::Dimensions;
use libc::{STDOUT_FILENO, TIOCGWINSZ};
use std::error::Error;

/// Clears the screen and moves the cursor to the top left corner.
pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

/// Moves the cursor to the top left corner.
pub fn move_cursor_to_bottom_right() {
    print!("{esc}[999C{esc}[999B", esc = 27 as char);
}

/// Gets the dimensions of the terminal.
pub fn get_dimensions() -> Result<Dimensions, Box<dyn Error>> {
    unsafe {
        let winsize = libc::ioctl(STDOUT_FILENO, TIOCGWINSZ);

        if winsize == -1 || winsize == 0 {
            move_cursor_to_bottom_right();
        } else {
            panic!("WHAT?");
        }

        dbg!(winsize);
    }

    Ok(Dimensions {
        height: 0,
        width: 0,
    })
}

// pub fn draw(config: &EditorConfig) {
//     for _ in 0..config.dimensions.height {
//         print!("~\r\n");
//     }
// }
