#![no_std] // Don't include the rust stdlib
#![no_main] // Disable rust entrypoints

use core::panic::PanicInfo;

static HELLO: &[u8] = b"HELLO WORLD!";

#[no_mangle] // Don't mangel this functions name
pub extern "C" fn _start() -> ! {
    // Entry point of the kernel
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// Called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
