#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

	mos::init();

    #[cfg(test)]
    test_main();

	println!("Yay no crashes!");
    loop {
		use mos::print;
		print!("-");
	}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mos::test_panic_handler(info)
}
