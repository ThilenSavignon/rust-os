#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// This function is called on Panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");
	panic!("Some panic message");

	loop {}
}
