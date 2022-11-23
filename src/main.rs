#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mos::{println, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

	mos::init();

    #[cfg(test)]
    test_main();

	println!("Yay no crashes!");

	let mut x = 0;

	while true{
		//println!("{}", x);
		x+=1;
		if x == 1000000 {
			println!("Yay counted to 1.000.000!!!");
			break;
		}
	}
	mos::hlt_loop()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mos::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mos::test_panic_handler(info)
}
