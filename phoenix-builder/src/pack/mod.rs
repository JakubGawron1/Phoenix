pub mod aarch64_pack;
pub mod x86_pack;

use crate::{Arch, BuildType};

use std::error::Error;

pub fn package_files(arch: Arch, build_type: BuildType)-> Result<(), Box<dyn Error>> {

    if build_type == BuildType::Release{
        if arch == Arch::AArch64{
            aarch64_pack::release_package_files().unwrap();
        }
        else{
            x86_pack::release_package_files().unwrap();
        }
    }
    else{
        if arch == Arch::AArch64{
            aarch64_pack::debug_package_files().unwrap();
        }
        else{
            x86_pack::debug_package_files().unwrap();
        }
    }

    Ok(())
}