#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	let mut x = 0;
	loop {
		println!("{}", x);
		x = x + 1;
		if x == 10000 {
			println!("yay counted to 10.000!!!");
			break;
		}
	}
	loop {}
}