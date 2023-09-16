#![no_std]

use core::{
    fmt,
    fmt::{Display, Write},
    sync::{atomic, atomic::compiler_fence},
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

/// Writes bytes directly to no$gba's debug register.
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

/// Implements the logging interface for no$gba.
///
/// This struct implements `log::Log`, allowing it to be used as a logger with the `log` crate.
/// Logging can be done using the standard log interface.
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

/// An error occurring during initialization.
#[derive(Debug)]
pub enum Error {
    /// The program is not running in no$gba.
    ///
    /// In many cases, this is a recoverable error. If this variant was returned by [`init()`],
    /// then the logger was never actually set, meaning a different logger could potentially be set
    /// instead.
    NotRunningInNoCashGba,

    /// An error returned by `log::set_logger()`.
    ///
    /// This most often indicates that another logger has already been set by the program.
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

/// A static logger instance.
///
/// When initializing with [`log::set_logger()`], a static reference to a logger must be provided.
/// This static logger can be used as the static reference.
static LOGGER: Logger = Logger;

/// Initialize no$gba logging.
///
/// # Errors
/// This function returns `Ok(())` if the logger was enabled. If the logger was not enabled for any
/// reason, it instead returns an [`Error`]. See the documentation for [`Error`] for what errors
/// can occur.
pub fn init() -> Result<(), Error> {
    if unsafe { NOCASH_GBA_SIGNATURE_ADDRESS.read_volatile() } != NOCASH_GBA_SIGNATURE {
        return Err(Error::NotRunningInNoCashGba);
    }

    // Disable interrupts, storing the previous value.
    //
    // This prevents an interrupt handler from attempting to set a different logger while
    // `log::set_logger()` is running.
    //
    // Compiler fences are used to prevent these function calls from being reordered during
    // compilation.
    let previous_ime = unsafe { IME.read_volatile() };
    // SAFETY: This is guaranteed to be a valid write.
    unsafe { IME.write_volatile(false) };
    compiler_fence(atomic::Ordering::Acquire);

    let result = unsafe { log::set_logger_racy(&LOGGER) }
        .map(|()| unsafe { log::set_max_level_racy(LevelFilter::Trace) })
        .map_err(Into::into);

    compiler_fence(atomic::Ordering::Release);
    // Restore previous interrupt enable value.
    // SAFETY: This is guaranteed to be a valid write.
    unsafe {
        IME.write_volatile(previous_ime);
    }

    result
}
