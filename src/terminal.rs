use std::{
    io::{stdin, Error},
    os::fd::AsRawFd,
};

use termios::{tcsetattr, Termios, ECHO, ICANON, TCSAFLUSH};

pub struct Terminal {}

impl Terminal {
    pub fn to_raw_mode() -> Result<(), Error> {
        let mut term = Termios::from_fd(stdin().as_raw_fd())?;
        term.c_lflag &= !(ECHO | ICANON);
        tcsetattr(stdin().as_raw_fd(), TCSAFLUSH, &term)?;
        Ok(())
    }

    pub fn to_cooked_mode() -> Result<(), Error> {
        let mut term = Termios::from_fd(stdin().as_raw_fd())?;
        term.c_lflag |= ECHO | ICANON;
        tcsetattr(stdin().as_raw_fd(), TCSAFLUSH, &term)?;
        Ok(())
    }
}
