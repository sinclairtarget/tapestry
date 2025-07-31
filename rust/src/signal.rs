use std::mem;
use std::ptr;

use crate::error::TapestryError;

static mut GOT_SIGWINCH: bool = false;

pub fn register_sigwinch_handler() -> Result<(), TapestryError> {
    unsafe {
        let mut sa: libc::sigaction = mem::zeroed();
        sa.sa_sigaction = (handle_sigwinch as *const ()) as usize;
        libc::sigemptyset(&mut sa.sa_mask);
        sa.sa_flags = libc::SA_RESTART;

        if libc::sigaction(libc::SIGWINCH, &sa, ptr::null_mut()) < 0 {
            return Err(TapestryError::new("sigaction() failed".to_string()));
        }
    }

    eprintln!("registered sigwinch handler");
    Ok(())
}

pub fn consume_sigwinch() -> bool {
    unsafe {
        let val = GOT_SIGWINCH;
        // <--- If the value gets set to true right here, we lose it. Oh well
        GOT_SIGWINCH = false;
        val
    }
}

fn handle_sigwinch() {
    unsafe {
        GOT_SIGWINCH = true;
    }
} 
