#![no_std] // Don't include the rust stdlib
#![no_main] // Disable rust entrypoints
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle] // Don't mangel this functions name
pub extern "C" fn _start() -> ! {
    // Entry point of the kernel
    println!("Hello world{}", "!");

    loop {}
}

// Called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
