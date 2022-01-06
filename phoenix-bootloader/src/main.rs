#![feature(abi_efiapi)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn efi_main()-> !{
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo)-> !{
    loop{}
}