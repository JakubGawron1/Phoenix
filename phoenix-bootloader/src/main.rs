#![feature(
    abi_efiapi,
    format_args_nl
)]
#![no_std]
#![no_main]

mod arch;

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;

use core::panic::PanicInfo;
use core::fmt::Write;
use alloc::string::String;
use alloc::vec::Vec;

use uefi::proto::console::text::Output;
use uefi::{prelude::*, table::boot::*};
use uefi_services::system_table;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, FrameBuffer, PixelFormat};
use uefi::table::boot::BootServices;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};

const ASCII_INTRO: &str = r"     
              ,,\\\                     ///,,       
            (\\\\//                     \\////)      
           (-(__//                       \\__)-)     
         ((-(__||                         ||__)-))    
        ((-(-(_||           ```\__        ||_)-)-))   
       ((-(-(/(/\\        ''; 9.- `      //\)\)-)-))   
       (-(-(/(/(/\\      '';;;;-\~      //\)\)\)-)-)   
       ((-(-(/(/(/\======,:;:;:;:,======/\)\)\)-)-))   
       '(((-(/(/(/(//////:%%%%%%%:\\\\\\)\)\)\)-)))`  
        '((-(/(/(/('uuuu:WWWWWWWWW:uuuu`)\)\)\)-))`   
         '((-(/(/(/('|||:wwwwwwwww:|||')\)\)\)-))`    
           '((((/(/('uuu:WWWWWWWWW:uuu`)\)\))))`     
             '':::UUUUUU:wwwwwwwww:UUUUUU:::``     
                  '''''''\uuuuuuuu/``````         
                         `JJJJJJJJJ`           
                          LLLLLLLLLL         
                         ///|||||||\\\       
                        (/(/(/(^)\)\)\)   
";
const SPLASH_INTRO: &'static [u8] = include_bytes!("../../other/splash.bmp");

static mut BOOT_SYSTEM_TABLE: Option<SystemTable<Boot>> = None;
static mut IMAGE: Option<Handle> = None;


#[no_mangle]
extern "C" fn efi_main(
  image: Handle, 
  mut system_table: SystemTable<Boot>, 
  rev: uefi::table::Revision
)-> Status{
    uefi_services::init(&mut system_table).expect_success("Failed to initialize utilities");
   
    system_table.stdout().clear().expect("Failed to clean screen");

    arch::main(image, &mut system_table);
 
    system_table.stdout().clear().expect("Failed to clean screen");
 


    writeln!(system_table.stdout(), "{}", ASCII_INTRO);
    writeln!(system_table.stdout(), "Phoenix Bootloader v{}, created by {}",env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    
    loop{}
}

fn check_revision(rev: uefi::table::Revision){
  let (major, minor) = (rev.major(), rev.minor());

  log::info!("UEFI {}.{}", major, minor / 10);

  assert!(major >= 2, "Running on an old, unsupported version of UEFI");
  assert!(
      minor >= 30,
      "Old version of UEFI 2, some features might not be available."
  );

}

