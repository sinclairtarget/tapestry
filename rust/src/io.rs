use crate::error::TapestryError;
use std::io;
use std::io::prelude::*;

pub fn read_key(key: &mut u8) -> Result<usize, TapestryError> {
    let mut buf = [0; 1];
    let n = io::stdin().read(&mut buf)?;
    *key = buf[0];
    Ok(n)
}

pub fn write(b: &[u8]) -> Result<(), TapestryError> {
    let _ = io::stdout().write(b)?;
    io::stdout().flush()?;
    Ok(())
}
