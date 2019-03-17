// Rust language features implementations

use core::panic::PanicInfo;
use core::alloc::Layout;
use log::*;
use crate::backtrace;

#[lang = "eh_personality"] 
extern fn eh_personality() {
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("\n\n{}", info);
    backtrace::backtrace();
    loop { crate::arch::cpu::halt() }
}

#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}
