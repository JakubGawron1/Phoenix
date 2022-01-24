use alloc::vec::Vec;

use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::table::boot::BootServices;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use tinybmp::Bmp;

pub fn test(_image: Handle, bt: &BootServices) {
    log::info!("Running graphics output protocol test");    

    if let Ok(gop) = bt.locate_protocol::<GraphicsOutput>() {
        let gop = gop.expect("Warnings encountered while opening GOP");
        let gop = unsafe { &mut *gop.get() };

        set_graphics_mode(gop);
        fill_color(gop);
        draw_bmp(gop);

        // Pause for 10 seconds to allow time to admire the result.
        bt.stall(10_000_000); 
        
        
        
       // set_graphics_mode(gop);

    } else {
        // No tests can be run.
        log::warn!("UEFI Graphics Output Protocol is not supported");
    }
    
}

// Set a larger graphics mode.
pub fn set_graphics_mode(gop: &mut GraphicsOutput) {
    // We know for sure QEMU has a 1024x768 mode.
    let mode = gop
        .modes()
        .map(|mode| mode.expect("Warnings encountered while querying mode"))
        .find(|mode| {
            let info = mode.info();
            info.resolution() == (1024, 768)
        })
        .unwrap();

    gop.set_mode(&mode)
        .expect_success("Failed to set graphics mode");
}

// Fill the screen with color.
fn fill_color(gop: &mut GraphicsOutput) {
    let op = BltOp::VideoFill {
        // Cornflower blue.
        color: BltPixel::new(100, 149, 237),
        dest: (0, 0),
        dims: (1024, 768),
    };

    gop.blt(op)
        .expect_success("Failed to fill screen with color");
}

fn draw_bmp(gop: &mut GraphicsOutput) {
    // Embed a BMP.
    let bmp_data = crate::SPLASH_INTRO;
  
    // Parse the BMP data.
    let bmp = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();
  
    // Convert width/height to usize.
    let width: usize = bmp.as_raw().size().width.try_into().unwrap();
    let height: usize = bmp.as_raw().size().height.try_into().unwrap();
  
    // Convert the pixel data into a form expected by the blit operation.
    let mut buffer = Vec::with_capacity(width * height);
    for pixel in bmp.pixels() {
        let color = pixel.1;
        buffer.push(BltPixel::new(color.r(), color.g(), color.b()));
    }
  
    // Blit the buffer to the framebuffer.
    gop.blt(BltOp::BufferToVideo {
        buffer: &buffer,
        src: BltRegion::Full,
        dest: (0, 0),
        dims: (width, height),
    })
    .expect_success("Failed to draw bmp");
}