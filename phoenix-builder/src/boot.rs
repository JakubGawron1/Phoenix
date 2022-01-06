use std::process::Command;

use crate::{BuildType, CARGO, Arch};

pub fn build_bootloader(arch: Arch, build_type: BuildType){
    let mut build_command = Command::new(CARGO);

    let mut  target_arch: &str = "x86_64";

    if arch == Arch::AArch64{
        target_arch = "aarch64";
    }

    let target: &str = "-unknown-uefi";

    let  target_triple = format!("{}{}", target_arch, target);

    build_command.current_dir("phoenix-bootloader");

    build_command.arg("build");
    
    build_command.arg("--target").arg(target_triple);

    if build_type == BuildType::Release{
        build_command.arg("--release");
    }

    if !build_command
        .status()
        .expect(&format!("Failed to run {:#?}", build_command))
        .success()
        {
            panic!("Failed to build bootloader");
        }
}