use libc::{
    termios as Termios, BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST,
};
use std::{
    io::{self, Write},
    mem,
};

use crate::helpers::convert_syserr_to_result;


pub struct RawTerminal<W: Write> {
    original_termios: Termios,
    // TODO: make private
    pub output: W,
}

impl<W: Write> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        let _ = set_terminal_attr(&self.original_termios);
    }
}

pub trait IntoRawMode: Write + Sized {
    fn try_into_raw_mode(self) -> io::Result<RawTerminal<Self>>;
}

impl<W: Write> IntoRawMode for W {
    fn try_into_raw_mode(self) -> io::Result<RawTerminal<W>> {
        let mut termios = get_terminal_attr()?;

        make_raw_terminal_attr(&mut termios);
        set_terminal_attr(&termios)?;

        Ok(RawTerminal {
            original_termios: termios,
            output: self,
        })
    }
}

/// Gets the current terminal attributes.
pub fn get_terminal_attr() -> io::Result<Termios> {
    let mut termios = unsafe { mem::zeroed() };

    convert_syserr_to_result(unsafe { libc::tcgetattr(libc::STDOUT_FILENO, &mut termios).into() })?;

    Ok(termios)
}

/// Sets the current terminal attributes.
pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    convert_syserr_to_result(unsafe {
        libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, termios).into()
    })
}

/// Sets the terminal to raw mode.
pub fn make_raw_terminal_attr(termios: &mut Termios) {
    termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    termios.c_oflag &= !(OPOST);
    termios.c_cflag |= CS8;
    termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
}
