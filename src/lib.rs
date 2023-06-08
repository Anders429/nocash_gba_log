#![no_std]

use core::{fmt, fmt::Display};

/// Location to check for the signature.
const NOCASH_GBA_SIGNATURE_ADDRESS: *const [u8; 7] = 0x04FFFA00 as *const [u8; 7];

/// This signature must be returned by the emulator for the logger to be enabled.
const NOCASH_GBA_SIGNATURE: [u8; 7] = *b"no$gba ";

#[derive(Debug)]
pub enum Error {
    NotRunningInNoCashGba,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotRunningInNoCashGba => fmt.write_str("not running in no$gba"),
        }
    }
}

pub fn init() -> Result<(), Error> {
    if unsafe { NOCASH_GBA_SIGNATURE_ADDRESS.read_volatile() } != NOCASH_GBA_SIGNATURE {
        return Err(Error::NotRunningInNoCashGba);
    }
    todo!("initialize the logger")
}
