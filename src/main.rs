#![no_std] // Don't include the rust stdlib
#![no_main] // Disable rust entrypoints

use core::panic::PanicInfo;

#[no_mangle] // Don't mangel this functions name
pub extern "C" fn _start() -> ! {
    // Entry point of the kernel
    loop {}
}

// Called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
