#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world from the darkness of borked.\n");
    println!("...");
    println!("Take that David!");
    println!("P.S. Borked is a great name thanks. :)");
    println!("\n\n\nAlso I have to say embedded programming is really fun!");
    println!("I will remove that if this dont fucking compile...");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
