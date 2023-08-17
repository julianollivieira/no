use libc::{
    termios as Termios, BRKINT, CS8, ECHO, ICANON, ICRNL, IEXTEN, INPCK, ISIG, ISTRIP, IXON, OPOST,
};
use std::{io, mem};

/// Sets the terminal to raw mode.
pub fn set_terminal_raw_mode() {
    let mut termios = get_terminal_attr().unwrap();
    make_raw_terminal_attr(&mut termios);
    set_terminal_attr(&termios);
}

/// Get the current terminal attributes.
///
/// TODO: error handling
pub fn get_terminal_attr() -> io::Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        libc::tcgetattr(libc::STDOUT_FILENO, &mut termios);
        Ok(termios)
    }
}

/// Set the current terminal attributes.
///
/// TODO: error handling
pub fn set_terminal_attr(termios: &Termios) {
    unsafe {
        libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, termios);
    }
}

/// Set the terminal to raw mode.
///
/// TODO: checkout (https://linux.die.net/man/3/cfmakeraw) for more flags
fn make_raw_terminal_attr(termios: &mut Termios) {
    termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    termios.c_oflag &= !(OPOST);
    termios.c_cflag |= CS8;
    termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
}
