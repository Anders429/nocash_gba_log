#![no_std]

use core::{
    fmt,
    fmt::{Display, Write},
};
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

/// Location to check for the signature.
const NOCASH_GBA_SIGNATURE_ADDRESS: *const [u8; 7] = 0x04FFFA00 as *const [u8; 7];
/// Address to write log messages to.
const NOCASH_GBA_DEBUG: *mut u8 = 0x04FFFA1C as *mut u8;
/// Interrupt Master Enable.
///
/// This register allows enabling and disabling interrupts.
const IME: *mut bool = 0x0400_0208 as *mut bool;

/// This signature must be returned by the emulator for the logger to be enabled.
const NOCASH_GBA_SIGNATURE: [u8; 7] = *b"no$gba ";

#[derive(Debug)]
struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                NOCASH_GBA_DEBUG.write_volatile(byte);
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // Disable interrupts, storing the previous value.
        //
        // This prevents synchronization issues when messages are logged in interrupt handling.
        // Interrupts triggered during this time will be handled when interrupts are reenabled.
        let previous_ime = unsafe { IME.read_volatile() };
        unsafe { IME.write_volatile(false) };

        write!(Writer, "[{:<5}]: {}\n", record.level(), record.args())
            .unwrap_or_else(|error| panic!("write to no$gba debug buffer failed: {}", error));

        // Restore previous interrupt enable value.
        unsafe {
            IME.write_volatile(previous_ime);
        }
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
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .map_err(Into::into)
}
