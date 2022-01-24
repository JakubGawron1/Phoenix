use uefi::ResultExt;
use uefi::prelude::*;
use uefi::proto::console::text::{Color, Output};

pub fn set_text_color(stdout: &mut Output){
    stdout
        .set_color(Color::Green, Color::Black)
        .expect_success("Failed to change console color");
    stdout.clear().expect_success("Failed to clear screen");
}

pub fn center_text(stdout: &mut Output) {
    // Move the cursor.
    // This will make this `info!` line below be (somewhat) centered.
    stdout
        .enable_cursor(true)
        .warning_as_error()
        .unwrap_or_else(|e| match e.status() {
            Status::UNSUPPORTED => info!("Cursor visibility control unavailable"),
            _ => panic!("Failed to show cursor"),
        });
    stdout
        .set_cursor_position(24, 0)
        .expect_success("Failed to move cursor");
        log::info!("what?");
    stdout
        .enable_cursor(false)
        .warning_as_error()
        .unwrap_or_else(|e| match e.status() {
            Status::UNSUPPORTED => info!("Cursor visibility control unavailable"),
            _ => panic!("Failed to hide cursor"),
        });
}