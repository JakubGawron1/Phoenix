use uefi::ResultExt;
use uefi::proto::console::text::{Color, Output};

pub fn set_text_color(stdout: &mut Output){
    stdout
        .set_color(Color::Green, Color::Black)
        .expect_success("Failed to change console color");
    stdout.clear().expect_success("Failed to clear screen");
}