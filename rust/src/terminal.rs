use std::mem;

use libc;
use libc::termios as Termios;

use crate::error::TapestryError;
use crate::escape;
use crate::io::write;

pub fn set_up(
    allow_private: bool,
) -> Result<impl FnOnce() -> Result<(), TapestryError>, TapestryError> {
    eprintln!("running terminal set up");

    if allow_private {
        write(escape::PRIVATE_SAVE_SCREEN)?;
        write(escape::PRIVATE_SAVE_CURSOR)?;
        write(escape::PRIVATE_HIDE_CURSOR)?;
    }

    let original_termios = enable_raw_mode()?;
    reset()?;

    Ok(move || tear_down(original_termios, allow_private))
}

pub fn get_dimensions() -> Result<(u16, u16), TapestryError> {
    let mut size = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let result = unsafe {
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ.into(), &mut size)
    };
    if result == -1 {
        return Err(TapestryError::new("ioctl() failed".to_string()));
    }

    Ok((size.ws_row, size.ws_col))
}

fn tear_down(
    original: Termios,
    allow_private: bool,
) -> Result<(), TapestryError> {
    eprintln!("running terminal tear down");

    if allow_private {
        write(escape::PRIVATE_SHOW_CURSOR)?;
        write(escape::PRIVATE_RESTORE_CURSOR)?;
        write(escape::PRIVATE_RESTORE_SCREEN)?;
    } else {
        reset()?;
    }

    disable_raw_mode(original)?;

    Ok(())
}

fn enable_raw_mode() -> Result<Termios, TapestryError> {
    let original: Termios = unsafe {
        let mut original = mem::zeroed();
        let result = libc::tcgetattr(libc::STDOUT_FILENO, &mut original);
        if result < 0 {
            return Err(TapestryError::new("tcgetattr() failed".to_string()));
        }

        original
    };

    let mut copy = original.clone();
    copy.c_iflag &=
        !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
    copy.c_oflag &= !libc::OPOST;
    copy.c_cflag |= libc::CS8;
    copy.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);

    copy.c_cc[libc::VMIN] = 0; // min bytes read() nees to return
    copy.c_cc[libc::VTIME] = 1; // max read() wait time in 10ths of a second

    unsafe {
        let result =
            libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSADRAIN, &copy);

        if result < 0 {
            return Err(TapestryError::new("tcsetattr() failed".to_string()));
        }
    }

    eprintln!("raw mode enabled");
    Ok(original)
}

fn disable_raw_mode(original: Termios) -> Result<(), TapestryError> {
    unsafe {
        let result =
            libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSADRAIN, &original);

        if result < 0 {
            return Err(TapestryError::new("tcsetattr() failed".to_string()));
        }
    }

    eprintln!("raw mode disabled");
    Ok(())
}

fn reset() -> Result<(), TapestryError> {
    write(escape::ERASE_ALL)?;
    write(escape::CURSOR_HOME)?;
    Ok(())
}
