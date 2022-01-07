use std::process::Command;

use crate::{BuildType, CARGO, Arch};

pub fn build_kernel(arch: Arch, build_type: BuildType){
    let mut build_command = Command::new(CARGO);

    let mut  target_arch = "x86_64";

    if arch == Arch::AArch64{
        target_arch = "aarch64";
    }

    let target_directory: &str = ".cargo/";

    let target: &str = "-phoenix.json";

    let target_triple = format!("./cargo/{}-phoenix.json", target_arch);

    build_command.current_dir("phoenix-kernel");

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
            panic!("Failed to build kernel");
        }
}