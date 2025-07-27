use std::io;

use libc;
use nix::sys::termios;

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
    original: termios::Termios,
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

fn enable_raw_mode() -> Result<termios::Termios, TapestryError> {
    let Ok(original) = termios::tcgetattr(io::stdout()) else {
        return Err(TapestryError::new("tcgetattr() failed".to_string()));
    };

    let mut copy = original.clone();
    copy.input_flags &= !(termios::InputFlags::BRKINT
        | termios::InputFlags::ICRNL
        | termios::InputFlags::INPCK
        | termios::InputFlags::ISTRIP
        | termios::InputFlags::IXON);
    copy.output_flags &= !termios::OutputFlags::OPOST;
    copy.control_flags |= termios::ControlFlags::CS8;
    copy.local_flags &= !(termios::LocalFlags::ECHO
        | termios::LocalFlags::ICANON
        | termios::LocalFlags::IEXTEN
        | termios::LocalFlags::ISIG);

    copy.control_chars[termios::SpecialCharacterIndices::VMIN as usize] = 0;
    copy.control_chars[termios::SpecialCharacterIndices::VTIME as usize] = 1;

    if let Err(_) =
        termios::tcsetattr(io::stdout(), termios::SetArg::TCSADRAIN, &copy)
    {
        return Err(TapestryError::new("tcsetattr() failed".to_string()));
    };

    eprintln!("raw mode enabled");
    Ok(original)
}

fn disable_raw_mode(original: termios::Termios) -> Result<(), TapestryError> {
    if let Err(_) =
        termios::tcsetattr(io::stdout(), termios::SetArg::TCSADRAIN, &original)
    {
        return Err(TapestryError::new("tcsetattr() failed".to_string()));
    };

    eprintln!("raw mode disabled");
    Ok(())
}

fn reset() -> Result<(), TapestryError> {
    write(escape::ERASE_ALL)?;
    write(escape::CURSOR_HOME)?;
    Ok(())
}
