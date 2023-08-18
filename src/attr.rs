use libc::{
    termios as Termios, BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST,
};
use std::{
    io::{self, Write},
    mem,
};

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
    let ret = unsafe { libc::tcgetattr(libc::STDOUT_FILENO, &mut termios) };

    ret.eq(&0)
        .then(|| termios)
        .ok_or_else(|| io::Error::last_os_error())
}

/// Sets the current terminal attributes.
pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
    let ret = unsafe { libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, termios) };

    ret.eq(&0)
        .then(|| ())
        .ok_or_else(|| io::Error::last_os_error())
}

/// Sets the terminal to raw mode.
pub fn make_raw_terminal_attr(termios: &mut Termios) {
    termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    termios.c_oflag &= !(OPOST);
    termios.c_cflag |= CS8;
    termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
    // termios.c_cc[libc::VMIN] = 0;
    // termios.c_cc[libc::VTIME] = 1;
}
