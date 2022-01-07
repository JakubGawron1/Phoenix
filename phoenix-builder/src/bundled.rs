use crate::{BUNDLED_DIR, PREBUILT_OVMF_URL, OVMF_FILES};

use std::fs;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::Write;

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