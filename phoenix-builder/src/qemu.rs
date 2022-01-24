use crate::{Arch, Graphic, BUNDLED_DIR, Bootloader, BuildType};

use std::process::Command;
use std::error::Error;

pub fn run_qemu(arch: Arch, graphic: Graphic, bootloader: Bootloader, build_type: BuildType)-> Result<(), Box<dyn Error>> {
    let mut project_arch: &str = "x86_64";
    let mut build_dir: &str = "x86_64";
    let mut image: &str = "DEBUGX64_OVMF.fd";
    let mut build_profile: &str = "debug";

    if arch == Arch::AArch64{
        project_arch = "aarch64";
        build_dir = "aarch64";
        image = "DEBUGAARCH64_QEMU_EFI.fd";
    }
    if bootloader == Bootloader::Limine{
        build_dir = "x86_limine";
    }
    if build_type == BuildType::Release{
        build_profile = "release"
    }
    let  qemu_binary = format!("qemu-system-{}", project_arch);

    let uefi_image = format!("{}/ovmf/{}",BUNDLED_DIR, image);

    let phoenix_image = format!("build/{}/{}/phoenix.img",build_dir, build_profile);

    let phoenix_image_command = format!("format=raw,file={}", phoenix_image);

    let mut run_command = Command::new(qemu_binary);

    if arch == Arch::AArch64{

        run_command.arg("-machine").arg("virt");

        run_command.arg("-cpu").arg("cortex-a72");
    }

    if  graphic == Graphic::Nographic{
        run_command.arg("-nographic");
    }

    run_command.arg("-bios").arg(uefi_image);

    run_command.arg("-drive").arg(phoenix_image_command);

    if arch == Arch::X86{
        #[cfg(target_os="linux")]
        run_command.arg("-enable-kvm");
    }

    if !run_command
        .status()
        .expect(&format!("Failde to run {:#?}", run_command))
        .success()
    {
        panic!("Failed to run qemu");
    }


 Ok(())


}