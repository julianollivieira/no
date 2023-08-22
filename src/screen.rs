use libc::{STDOUT_FILENO, TIOCGWINSZ};
use std::{ffi::c_ushort, io, mem};

use crate::helpers::convert_syserr_to_result;

/// Escape character.
const ESC: char = 27 as char;

/// Represents the size of the terminal.
#[repr(C)]
struct TerminalSize {
    row: c_ushort,
    col: c_ushort,
    x: c_ushort,
    y: c_ushort,
}

/// Move cursor to origin.
pub fn move_cursor_to_origin(w: &mut impl io::Write) -> io::Result<()> {
    write!(w, "{ESC}[H")
}

/// Move cursor to specified coordinates.
pub fn move_cursor_to_(w: &mut impl io::Write, row: u16, col: u16) -> io::Result<()> {
    // write!(w, "{ESC}[{row};{col}H")
    write!(w, "{ESC}[5;5H")
}

/// Clears the current line.
pub fn clear_line(w: &mut impl io::Write) -> io::Result<()> {
    write!(w, "{ESC}[K")
}

/// Hides the cursor.
pub fn hide_cursor(w: &mut impl io::Write) -> io::Result<()> {
    write!(w, "{ESC}[?25l")
}

/// Shows the cursor.
pub fn show_cursor(w: &mut impl io::Write) -> io::Result<()> {
    write!(w, "{ESC}[?25h")
}

/// Gets the dimensions of the terminal.
///
/// TODO: provide fallback method to retrieve terminal size using cursor
/// position
pub fn get_terminal_size() -> io::Result<(u16, u16)> {
    let mut size: TerminalSize = unsafe { mem::zeroed() };

    convert_syserr_to_result(unsafe {
        libc::ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size as *mut _).into()
    })?;

    Ok((size.row, size.col))
}

/// Render buffer to the screen.
pub fn render(w: &mut impl io::Write, buf: &Vec<u8>) -> io::Result<()> {
    write!(w, "{}", String::from_utf8_lossy(buf))
}
