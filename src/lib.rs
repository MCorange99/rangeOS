#![feature(lang_items)]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![no_std]

// normal use's
use core::panic::PanicInfo;

// Local modules
#[macro_use] 
mod vga_buff;
mod interrupts;
mod gdt;
mod utils;

pub fn init() {
    vga_buff::clear_screen();
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); 
    
}

#[no_mangle]
pub extern fn rust_main(mb_info_ptr: usize) {
    init();
    println!("Hello World{}", "!");

    let _boot_info = unsafe{ multiboot2::load(mb_info_ptr) };

    utils::hlt_loop()
}

#[panic_handler]
pub extern fn panic(_s: &PanicInfo) -> ! {
    println!("{}", _s);
    utils::hlt_loop()
}