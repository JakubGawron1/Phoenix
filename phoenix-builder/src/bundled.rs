use crate::{BUNDLED_DIR, PREBUILT_OVMF_URL, OVMF_FILES, LIMINE_GITHUB_URL, LIMINE_RELEASE_BRANCH};

use std::fs;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;

async fn update_ovmf() -> Result<(), Box<dyn Error>> {
    let ovmf_out_dir = Path::new(BUNDLED_DIR).join("ovmf");

    fs::create_dir_all(&ovmf_out_dir)?;

    for ovmf_file in OVMF_FILES.iter() {
        println!("INFO: Downloading {}", ovmf_file);

        let response = reqwest::get(format!("{}{}", PREBUILT_OVMF_URL, ovmf_file)).await?;
        let bytes = response.bytes().await?;

        let mut output = File::create(ovmf_out_dir.join(ovmf_file))?;
        output.write_all(bytes.as_ref())?;
    }

    Ok(())
}

/// Run [update_ovmf] if the OVMF files do not exist.
///
/// **Note**: To update the existing OVMF files run `cargo kmake update`.
pub async fn download_ovmf_prebuilt() -> Result<(), Box<dyn Error>> {
    let ovmf_out_dir = Path::new(BUNDLED_DIR).join("ovmf");

    for ovmf_file in OVMF_FILES.iter() {
        if !ovmf_out_dir.join(ovmf_file).exists() {
            update_ovmf().await?;

            return Ok(());
        }
    }

    Ok(())
}

pub fn clean() -> Result<(), Box<dyn Error>> {

    xshell::rm_rf("phoenix-builder/Cargo.lock").unwrap();
    xshell::rm_rf("phoenix-bootloader/Cargo.lock").unwrap();
    xshell::rm_rf("phoenix-kernel/Cargo.lock").unwrap();
    xshell::rm_rf("build").unwrap();
    xshell::rm_rf("bundled").unwrap();
    xshell::rm_rf("phoenix-bootloader/target").unwrap();
    xshell::rm_rf("phoenix-kernel/target").unwrap();
    xshell::rm_rf("phoenix-builder/target").unwrap();
    

    Ok(())

}

pub async fn update_limine() -> Result<(), Box<dyn Error>> {
    let limine_out_dir = Path::new(BUNDLED_DIR).join("limine");
    let mut remove_in_windows = Command::new("remove_win.bat");

        xshell::rm_rf("bundled/limine").unwrap();

        fs::create_dir_all(&limine_out_dir)?;

        let mut limine_clone_cmd = Command::new("git");

        limine_clone_cmd.arg("clone").arg(LIMINE_GITHUB_URL);
        limine_clone_cmd.arg("-b").arg(LIMINE_RELEASE_BRANCH);

        limine_clone_cmd.arg("bundled/limine");

        if !limine_clone_cmd
            .status()
            .expect(&format!("Failed to run {:#?}", limine_clone_cmd))
            .success()
        {
            panic!("Failed to clone the latest prebuilt limine files")
        }
    

    Ok(())
}

/// Run [update_limine] if the limine prebuilt files do
/// not exist.
///
/// **Note**: To update the limine prebuilt files run `cargo boot update`.
pub async fn download_limine_prebuilt() -> Result<(), Box<dyn Error>> {
    let build_dir = Path::new(BUNDLED_DIR).join("limine");

    if !build_dir.exists() {
        update_limine().await?;

        return Ok(());
    }

    Ok(())
}
