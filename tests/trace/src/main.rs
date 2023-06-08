#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn main() {
    nocash_gba_log::init().expect("could not initialize no$gba logging");
    log::trace!("Hello, world!");

    loop {}
}
