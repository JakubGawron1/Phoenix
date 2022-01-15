#![feature(
    abi_efiapi,
    format_args_nl
)]
#![no_std]
#![no_main]



use core::panic::PanicInfo;
use core::fmt::Write;

use uefi::{prelude::*, table::boot::*};

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

static mut BOOT_SYSTEM_TABLE: Option<SystemTable<Boot>> = None;
static mut IMAGE: Option<Handle> = None;

#[no_mangle]
extern "C" fn efi_main(image: Handle, mut system_table: SystemTable<Boot>)-> Status{
    uefi_services::init(&mut system_table).expect_success("Failed to initialize utilities");

    let stdout = system_table.stdout();
    stdout.clear().unwrap().unwrap();
    writeln!(stdout, "{}", ASCII_INTRO).unwrap();
    writeln!(stdout, "Phoenix Bootloader v{}, created by {}",env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));

    Status::SUCCESS;
    loop{}
}

