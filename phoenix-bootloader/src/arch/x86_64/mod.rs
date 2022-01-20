pub mod gop;
pub mod stdout;

use uefi::prelude::*;
use uefi::table::boot::BootServices;
use uefi::Handle;
use uefi::proto::console::text::Output;

pub fn main(image: Handle, system_table: &mut SystemTable<Boot>){
    let stdout = system_table.stdout();

    stdout::set_text_color(stdout);

    let bt = system_table.boot_services();

    bt.stall(10_000_000); 

    gop::test(image, bt);

    system_table.stdout().clear().expect("Failed to clear screen");


} 

