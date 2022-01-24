use std::fs;
use std::io;

use std::error::Error;
use std::fs::File;
use std::io::Seek;
use std::path::Path;

use std::convert::TryFrom;



fn get_fat_filesystem_len(fat: Vec<&Path>) -> u64 {
    let mb = 1024 * 1024; // Size of a megabyte and round it to next megabyte.
    let mut size = 0x00;

    for file in fat {
        // Retrieve size of `path` file and round it up.
        let file_size = fs::metadata(file).unwrap().len();
        let file_size_rounded = ((file_size - 1) / mb + 1) * mb;

        size += file_size_rounded;
    }

    size
}

fn create_fat_filesystem(
    fat_path: &Path,
    efi_file: &Path,
    limine_path: &Path,
    kernel_file: &Path,
    limine_cfg: &Path,

) -> Result<(), Box<dyn Error>> {
    let fat = vec![efi_file, kernel_file];


    let fat_len = get_fat_filesystem_len(fat);

    // Create new filesystem image file at the given path and set its length.
    let fat_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&fat_path)?;

    fat_file.set_len(fat_len)?;

    // Create new FAT file system and open it.
    let format_options = fatfs::FormatVolumeOptions::new().fat_type(fatfs::FatType::Fat32);
    fatfs::format_volume(&fat_file, format_options)?;

    let filesystem = fatfs::FileSystem::new(&fat_file, fatfs::FsOptions::new())?;

    // Copy EFI file to FAT filesystem.
    let root_dir = filesystem.root_dir();

    root_dir.create_dir("EFI")?;
    root_dir.create_dir("EFI/BOOT")?;
    root_dir.create_dir("EFI/KERNEL")?;

    macro_rules! create_fat_file {
        ($name:ident => $path:expr) => {
            let mut $name = root_dir.create_file($path)?;
            $name.truncate()?;
        };
    }

    macro_rules! copy_contents_fat {
        ($path:expr => $name:ident) => {
            io::copy(&mut fs::File::open($path)?, &mut $name)?
        };
    }

    create_fat_file!(phoenix_bootx64 => "EFI/BOOT/phoenix-bootloader.efi");
    create_fat_file!(bootx64 => "EFI/BOOT/BOOTX64.EFI");
    create_fat_file!(kernel => "phoenix-kernel.elf");
    create_fat_file!(limine_conf => "limine.cfg");

    copy_contents_fat!(&efi_file => phoenix_bootx64);
    copy_contents_fat!(&limine_path => bootx64);
    copy_contents_fat!(&kernel_file => kernel);
    copy_contents_fat!(&limine_cfg => limine_conf);


    Ok(())
}

fn create_gpt_disk(disk_path: &Path, fat_image: &Path) -> Result<(), Box<dyn Error>> {
    // Create new file.
    let mut disk = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&disk_path)?;

    // Set file size.
    let partition_size: u64 = fs::metadata(&fat_image)?.len();
    let disk_size = partition_size + 1024 * 64; // For GPT headers.
    disk.set_len(disk_size)?;

    /*
     * Create a protective MBR at LBA0 so that disk is not considered
     * unformatted on BIOS systems
     */
    let mbr = gpt::mbr::ProtectiveMBR::with_lb_size(
        u32::try_from((disk_size / 512) - 1).unwrap_or(0xFF_FF_FF_FF),
    );

    mbr.overwrite_lba0(&mut disk)?;

    // Create new GPT structure.
    let block_size = gpt::disk::LogicalBlockSize::Lb512;

    let mut gpt = gpt::GptConfig::new()
        .writable(true)
        .initialized(false)
        .logical_block_size(block_size)
        .create_from_device(Box::new(&mut disk), None)?;

    gpt.update_partitions(Default::default())?;

    // Add new EFI system partition and get its byte offset in the file.
    let partition_id = gpt.add_partition("boot", partition_size, gpt::partition_types::EFI, 0, None)?;

    let partition = gpt
        .partitions()
        .get(&partition_id)
        .expect("Boot partition not found");
    let start_offset = partition.bytes_start(block_size)?;

    // Close the GPT structure and flush out the changes.
    gpt.write()?;

    // Place the FAT filesystem in the newly created partition.
    disk.seek(io::SeekFrom::Start(start_offset))?;
    io::copy(&mut File::open(&fat_image)?, &mut disk)?;

    Ok(())
}

/// Packages all of the files by creating the build directory and copying
/// the `phoenix-kernel.elf` and the `BOOTX64.EFI` files to the build directory and creating
/// fat file from the build directory.
pub fn debug_package_files() -> Result<(), Box<dyn Error>> {
    let efi_file = Path::new("phoenix-bootloader/target/x86_64-unknown-uefi/debug/phoenix-bootloader.efi");

    let kernel_file = Path::new("phoenix-kernel/target/x86_64-phoenix/debug/phoenix-kernel");
    let out_path = Path::new("build").join("x86_limine").join("debug");

    let limine_cfg = Path::new("phoenix-kernel/.cargo/limine.cfg");
    let limine_path = Path::new("bundled/limine/BOOTX64.EFI");

    let fat_path = out_path.join("phoenix.fat");
    let img_path = out_path.join("phoenix.img");

    fs::create_dir_all(out_path)?;

    create_fat_filesystem(&fat_path, &efi_file, limine_path, &kernel_file, limine_cfg)?;
    create_gpt_disk(&img_path, &fat_path)?;

    

    Ok(())
}

pub fn release_package_files() -> Result<(), Box<dyn Error>> {
    let efi_file = Path::new("phoenix-bootloader/target/x86_64-unknown-uefi/release/phoenix-bootloader.efi");
    let limine_path = Path::new("bundled/limine/BOOTX64.EFI");
    let kernel_file = Path::new("phoenix-kernel/target/x86_64-phoenix/release/phoenix-kernel");
    let limine_cfg = Path::new("phoenix-kernel/.cargo/limine.cfg");
    let out_path = Path::new("build").join("x86_limine").join("release");

    let fat_path = out_path.join("phoenix.fat");
    let img_path = out_path.join("phoenix.img");

    fs::create_dir_all(out_path)?;

    create_fat_filesystem(&fat_path, &efi_file, &limine_path, &kernel_file, &limine_cfg)?;
    create_gpt_disk(&img_path, &fat_path)?;

    

    Ok(())
}