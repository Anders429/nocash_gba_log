#[no_std]

/// Location to check for the signature.
const NOCASH_GBA_SIGNATURE_ADDRESS: *const [u8; 7] = 0x04FFFA00 as *const [u8; 7];

/// This signature must be returned by the emulator for the logger to be enabled.
const NOCASH_GBA_SIGNATURE: [u8; 7] = *b"no$gba ";

pub enum Error {
    NotRunningInNoCashGba,
}

pub fn init() -> Result<(), Error> {
    if unsafe { NOCASH_GBA_SIGNATURE_ADDRESS.read_volatile() } != NOCASH_GBA_SIGNATURE {
        return Err(Error::NotRunningInNoCashGba);
    }
    todo!("initialize the logger")
}
