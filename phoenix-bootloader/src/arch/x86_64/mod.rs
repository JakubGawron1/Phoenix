pub mod gop;
pub mod stdout;

use crate::{ASCII_IMAGE, ASCII_INTRO};

use uefi::prelude::*;
use uefi::Handle;
use uefi::proto::console::gop::GraphicsOutput;
use core::fmt::Write;

pub fn main(image: Handle, system_table: &mut SystemTable<Boot>){
    stdout::set_text_color(system_table.stdout());

    gop::test(image, system_table.boot_services());

    system_table.stdout().reset(true).expect_success("Failed to reset stdout");
    system_table.stdout().clear().expect_success("Failed to clear screem");

    stdout::center_text(system_table.stdout());
    write!(system_table.stdout(),"{}", ASCII_IMAGE).unwrap();
    writeln!(system_table.stdout(),"{} \n", ASCII_INTRO).unwrap();
    writeln!(system_table.stdout(),"Phoenix Bootloader v{} created by {}", env!("CARGO_PKG_VERSION"),env!("CARGO_PKG_AUTHORS")).unwrap();

    crate::check_revision(system_table.uefi_revision(), system_table);
}