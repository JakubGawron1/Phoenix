#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern "C" fn main_kernel()-> !{
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo)-> !{
    loop{}
}