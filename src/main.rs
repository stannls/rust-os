#![no_std] // Don't include the rust stdlib
#![no_main] // Disable rust entrypoints
mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"HELLO WORLD!";

#[no_mangle] // Don't mangel this functions name
pub extern "C" fn _start() -> ! {
    // Entry point of the kernel
    vga_buffer::print_something();

    loop {}
}

// Called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
