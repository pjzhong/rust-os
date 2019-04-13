#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_omports))]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::println;
use rust_os::{exit_qemu, serial_println};
use x86_64::{structures::paging::PageTable, VirtAddr};

//set the entry point
entry_point!(kernel_main);

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
             //pub extern "C" fn _start() -> ! {
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::interrupts::PICS;

    rust_os::gdt::init();
    rust_os::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    use rust_os::memory;
    use x86_64::{structures::paging::MapperAllSizes};

    let mapper = unsafe { memory::init(boot_info.physical_memory_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x20010a,
        // some stack page
        0x57ac_001f_fe48,
        //virtual address mappedto physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
        serial_println!("{:?} -> {:?}", virt, phys);
    }

    serial_println!("ok");
    unsafe {
        exit_qemu();
    }
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");

    serial_println!("{}", info);

    unsafe {
        exit_qemu();
    }
    loop {}
}
