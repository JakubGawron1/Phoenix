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

use core::fmt::Write;
use alloc::string::String;

use uefi::{prelude::*};

const ASCII_IMAGE: &str = r"                                                                                                          
              ,,\\\                     ///,,       
            (\\\\//                     \\////)      
           (-(__//                       \\__)-)     
         ((-(__||                         ||__)-))    
        ((-(-(_||           ```\__        ||_)-)-))   
       ((-(-(/(/\\        ''; 9.--'      //\)\)-)-))   
       (-(-(/(/(/\\      '';;;;-\~      //\)\)\)-)-)   
       ((-(-(/(/(/\======,:;:;:;:,======/\)\)\)-)-))   
       '(((-(/(/(/(//////:%%%%%%%:\\\\\\)\)\)\)-)))`  
        '((-(/(/(/('uuuu:WWWWWWWWW:uuuu`)\)\)\)-))`   
         '((-(/(/(/('|||:wwwwwwwww:|||')\)\)\)-))`    
           '((((/(/('uuu:WWWWWWWWW:uuu`)\)\))))`     
             '':::UUUUUU:wwwwwwwww:UUUUUU:::``     
                   '''''''uuuuuuu``````   
                         /JJJJJJJ\    
                        /LLLLLLLLL\   
                       ///|||||||\\\   
                      (/(/(/(^)\)\)\)
";

const ASCII_INTRO: &str = r"
 __        __   ___             __   __  
|__) |__| /  \ |__  |\ | | \_/ /  \ /__` 
|    |  | \__/ |___ | \| | / \ \__/ .__/ ";

const SPLASH_INTRO: &'static [u8] = include_bytes!("../../other/splash.bmp");

#[no_mangle]
extern "C" fn efi_main(
  image: Handle, 
  mut system_table: SystemTable<Boot>, 
)-> Status{
    uefi_services::init(&mut system_table).expect_success("Failed to initialize utilities");
   
    system_table.stdout().clear().expect("Failed to clean screen").unwrap();
 
    arch::main(image, &mut system_table);

    Status::SUCCESS.is_success();    
    
    loop{}
}

pub fn check_revision(rev: uefi::table::Revision, system_table: &mut SystemTable<Boot>){
  let (major, minor) = (rev.major(), rev.minor());
  
  let mut buf = String::new();
    system_table.firmware_vendor().as_str_in_buf(&mut buf).unwrap();

  writeln!(system_table.stdout(),"INFO: UEFI {}.{}", major, minor / 10).unwrap();
  writeln!(system_table.stdout(),"INFO: Firmware Vendor: {}", buf.as_str()).unwrap();

  assert!(major >= 2, "Running on an old, unsupported version of UEFI");
  assert!(
      minor >= 30,
      "Old version of UEFI 2, some features might not be available."
  );

}