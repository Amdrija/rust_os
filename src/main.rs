#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

use crate::vga_buffer::WRITER;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");
    loop {}
}
