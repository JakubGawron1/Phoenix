use crate::{ASCII_IMAGE, ASCII_INTRO};

use core::fmt::Write;

use uefi::proto::console::text::Output;
use uefi::Handle;
use uefi::prelude::*;
use cortex_a::registers::*;
use tock_registers::interfaces::Readable;

pub fn main(image: Handle, system_table: &mut SystemTable<Boot>){


    writeln!(system_table.stdout(),"{}",ASCII_IMAGE);
    writeln!(system_table.stdout(),"{} \n", ASCII_INTRO);
    writeln!(system_table.stdout(),"Phoenix Bootloader v{} created by {} \n", env!("CARGO_PKG_VERSION"),env!("CARGO_PKG_AUTHORS"));
    
    crate::check_revision(system_table.uefi_revision(), system_table);

    writeln!(system_table.stdout(),"INFO: CurrentEL: {}", CurrentEL.get() >> 2);

    debug_assert_eq!(CurrentEL.get() >> 1, 2);
}



#[no_mangle]
pub extern "C" fn __chkstk() {}