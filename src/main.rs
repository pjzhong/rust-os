#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


static HELLO: &[u8] = b"Hello ZJP!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
   /* let vga_buffer = 0xb8000 as * mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }*/

    println!("Hello World, I am {}", "zjp");
    panic!("Some panic message");
    loop {}
}
