#![no_std]

use core::{fmt, fmt::Display};
use log::{Log, Metadata, Record, SetLoggerError};

/// Location to check for the signature.
const NOCASH_GBA_SIGNATURE_ADDRESS: *const [u8; 7] = 0x04FFFA00 as *const [u8; 7];

/// This signature must be returned by the emulator for the logger to be enabled.
const NOCASH_GBA_SIGNATURE: [u8; 7] = *b"no$gba ";

#[derive(Debug)]
struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        todo!()
    }

    fn flush(&self) {}
}

#[derive(Debug)]
pub enum Error {
    NotRunningInNoCashGba,

    SetLoggerError(SetLoggerError),
}

impl From<SetLoggerError> for Error {
    fn from(error: SetLoggerError) -> Self {
        Self::SetLoggerError(error)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotRunningInNoCashGba => fmt.write_str("not running in no$gba"),
            Self::SetLoggerError(error) => write!(fmt, "`log::set_logger()` error: {error}"),
        }
    }
}

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), Error> {
    if unsafe { NOCASH_GBA_SIGNATURE_ADDRESS.read_volatile() } != NOCASH_GBA_SIGNATURE {
        return Err(Error::NotRunningInNoCashGba);
    }
    log::set_logger(&LOGGER).map_err(Into::into)
}
