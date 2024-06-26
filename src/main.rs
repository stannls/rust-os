#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use rust_os::{print, println, task::{keyboard, simple_executor::SimpleExecutor, Task}};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::{self, BootInfoFrameAllocator};
    use rust_os::allocator;
    use x86_64::VirtAddr;
    println!("Initialising kernel...");

    rust_os::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialisation failed.");
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));

    println!("Sucessfully initialised kernel...");
    print!(">");

    executor.run();

    #[cfg(test)]
    test_main(); 

    rust_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
