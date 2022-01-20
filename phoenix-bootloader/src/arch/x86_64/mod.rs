pub mod gop;
pub mod stdout;

use uefi::prelude::*;
use uefi::table::boot::BootServices;
use uefi::Handle;
use uefi::proto::console::text::Output;

pub fn main(image: Handle, system_table: &SystemTable<Boot>){
    let stdout = system_table.stdout();
    gop::test(image, system_table);

    system_table.stdout().clear().expect("Failed to clear screen");

    stdout::set_text_color(stdout);
}