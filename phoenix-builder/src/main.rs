mod boot;
mod kernel;
mod bundled;
mod pack;
mod tools;
mod qemu;

use std::{time::Instant, env};

use structopt::StructOpt;

const CARGO: &str = env!("CARGO");

const BUNDLED_DIR: &str = "bundled";

const PREBUILT_OVMF_URL: &str =
    "https://jakubgawron1.github.io/OVMF-nightly/bin/";

const LIMINE_GITHUB_URL: &str = "https://github.com/limine-bootloader/limine";
const LIMINE_RELEASE_BRANCH: &str = "latest-binary";


const OVMF_FILES: [&str; 2] = [
    "DEBUGX64_OVMF.fd",
    "DEBUGAARCH64_QEMU_EFI.fd",
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BuildType {
    Debug,
    Release,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Bootloader{
    Limine,
    Phoenix,
}
impl From<Option<String>> for Bootloader {
    fn from(bootloader: Option<String>) -> Self{
        match bootloader.as_deref() {
            Some("phoenix") | Some("Phoenix") => Self::Phoenix,
            Some("limine") => Self::Limine,
            Some(_v) => Self::Phoenix,
            None => panic!("Please specific architetcure"),

        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Arch {
    AArch64,
    X86,
}

impl From<Option<String>> for Arch {
    fn from(arch: Option<String>) -> Self{
        match arch.as_deref() {
            Some("aarch64") | Some("arm") | Some("aarch") | Some("arm64") => Self::AArch64,
            Some("x86") | Some("x86_64")  => Self::X86,
            Some(v) => panic!("Not supported architetcure {}", v),
            None => panic!("Please specific architetcure"),

        }
    }
}

#[derive(StructOpt, Debug)]
enum PhoenixCliCommand{
    Run{

        #[structopt(short, long)]
        release: bool,

        #[structopt(long)]
        arch: Option<String>,

        #[structopt(long)]
        bootloader: Option<String>
    },

    Build{

        #[structopt(short, long)]
        release: bool,

        #[structopt(long)]
        arch: Option<String>,

        #[structopt(long)]
        bootloader: Option<String>
    },
    Update,
    Format,
    Clean,

}

#[derive(Debug, StructOpt)]
struct PhoenixCli {
    #[structopt(subcommand)]
    command: Option<PhoenixCliCommand>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    let phoenix_command = PhoenixCli::from_args();

    match phoenix_command.command {
        Some(command) => match command{
            PhoenixCliCommand::Run{
                release,
                arch,
                bootloader,
            } => {
                
                let arch = Arch::from(arch);

                let bootloader = Bootloader::from(bootloader);

                let build_type = if release {
                    BuildType::Release
                } else {
                    BuildType::Debug
                };

                boot::build_bootloader(arch, build_type);

                kernel::build_kernel(arch, build_type);

                pack::package_files(arch, build_type, bootloader).unwrap();

                let now = Instant::now().elapsed();
                
                println!("Success built in {:?}", now);

                qemu::run_qemu(arch, bootloader).unwrap();
            }
            PhoenixCliCommand::Build{
                release,
                arch,
                bootloader,
            } => {
                
                let arch = Arch::from(arch);

                let bootloader = Bootloader::from(bootloader);

                let build_type = if release {
                    BuildType::Release
                } 
                else{
                    BuildType::Debug
                };

                boot::build_bootloader(arch, build_type);

                kernel::build_kernel(arch, build_type);

                pack::package_files(arch, build_type, bootloader).unwrap();

                let now = Instant::now().elapsed();

                println!("Success built in {:?}", now);
            }
            PhoenixCliCommand::Update => {
                bundled::download_ovmf_prebuilt().await.unwrap();
                bundled::download_limine_prebuilt().await.unwrap();
                
            }
            PhoenixCliCommand::Format => {
                println!("Heloo world");
            }
            PhoenixCliCommand::Clean => {
                bundled::clean().unwrap();
            }
        },
        None => {}
    }

    Ok(())
}
