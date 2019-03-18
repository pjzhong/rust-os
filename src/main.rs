#![cfg_attr(not(test), no_std)] //#![no_std]
#![cfg_attr(not(test), no_main)] // instead of `#![no_main]`
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use rust_os::println;
use rust_os::serial_println;

/// This function is called o panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default
#[cfg(not(test))]
#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World, I am {}", "zjp"); // write to vga, GUI

    rust_os::gdt::init();
    rust_os::interrupts::init_idt();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    // trigger a page fault
    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 42;
    //}

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // bootimage run -- -serial mod:stdio
    // write to serial
    // serial_println!("Hello Host, I am {}", "zjp");

    println!("It did not crash!");
    loop {}
}
