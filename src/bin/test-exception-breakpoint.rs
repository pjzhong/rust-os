#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macors, unused_imports))]

use core::panic::PanicInfo;
use rust_os::{exit_qemu, serial_println};

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::interrupts::init_idt();

    x86_64::instructions::interrupts::int3();

    serial_println!("ok");

    unsafe { exit_qemu(); }
    loop {};
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");

    serial_println!("{}", info);

    unsafe { exit_qemu(); }
    loop {}
}
