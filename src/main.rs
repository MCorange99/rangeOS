
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rangeos::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

mod vga_buffer;
mod utils;
mod serial;

// don't mangle the name of this function
#[no_mangle]

// this function is the entry point, since the linker looks for a function
// named `_start` by default
pub extern "C" fn _start() -> ! {
    use rangeos::print;
    println!("About fucking time{}", "!");
    rangeos::init();
    
    #[cfg(test)]
    test_main();
    
    rangeos::hlt_loop();  
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rangeos::hlt_loop();  
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rangeos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}