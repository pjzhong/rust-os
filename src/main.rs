#![cfg_attr(not(test), no_std)] //#![no_std]
#![cfg_attr(not(test), no_main)] // instead of `#![no_main]`
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use rust_os::println;
//use rust_os::{exit_qemu, serial_println};
use bootloader::{entry_point, BootInfo};
use x86_64::{structures::paging::PageTable, VirtAddr};

// set the entry_point
entry_point!(kernel_main);

/// This function is called o panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default
#[cfg(not(test))]
#[no_mangle] //don't mangle the name of this function
             //pub extern "C" fn _start() -> ! {*/
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::interrupts::PICS;
    println!("Hello World, I am {}", "zjp"); // write to vga, GUI

    rust_os::gdt::init();
    rust_os::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    //    fn stack_overflow() {
    //      stack_overflow(); // for each recursion, the return address is pushed
    // }

    // trigger a stack overflow
    // :tack_overflow();

    // trigger a page fault
    //unsafe {
    //    *(0xdeadbeef as *mut u64) = 42;
    //}

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // bootimage run -- -serial mod:stdio
    // write to serial
    // serial_println!("Hello Host, I am {}", "zjp")

    use rust_os::memory;
    use x86_64::structures::paging::Page;

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = memory::init_frame_allocator(&boot_info.memory_map);

    // map a previously unmapped page
    let page = Page::containing_address(VirtAddr::new(0x1000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    println!("It did not crash!");
    rust_os::hlt_loop();
}
