# Building PhoenixOS
> The default operating system for building PhoenixOS is any Linux distribution, but that doesn't mean you can't build it on other operating systems

## Prerequisites

- One of the operating systems:
    - Windows 10 or Windows 11
    - GNU Linux:
        - `Ubuntu` or `Debian` based distro (e.g `Ubuntu 20.04` LTS or `Ubuntu 21.10`)
        - `Arch` based distro (e.g `Arch Linux`, `Manjaro Linux`)
        - `RHEL` based distro (e.g `CentOS` or `Fedora Linux`)
    - MacOS BigSur or MacOS Monterey
- **At least 4GB of RAM*
- **30 GB of disk memory*
- Basic knowledge of the Command Prompt or Bash
> Requirements marked with an asterisk do not have to be met (yet)

## Requirements
> Below is a list of the required dependencies, which you will install later as described a little below

 - Rust 1.60-nightly
 - Qemu for X86_64 and AArch64 architectures
 - Git VSC
 - Make (optional)
 - Python (optional, but beneficial for `Windows`)

# Build on Linux
>Installing all the required dependencies on Linux is very simple. Just run Bash and paste the commands below.

## Instalation
To install the Rust programming language use this command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then follow the installer, and in case of problems with the installation see [this page](https://doc.rust-lang.org/book/ch01-01-installation.html)

You can install the rest of the dependencies with one command:

If you are using `Debian` based distro:
```bash
sudo apt install qemu-system-x86 qemu-system-arm make git
```

If you are using `Arch` based distro:
```bash
sudo pacman -S qemu qemu-arch-extra make git
```
If you are using `RHEL` based distro:
```bash
sudo dnf install qemu make git
```
## Preparations
> You are well on your way to building PhoenixOS, but there are a few things you need to do before building and running PhoenixOS

## 1. Get the source code
Use the command below to get the source code:
```bash
git clone https://github.com/JakubGawron1/Phoenix.git
```
## 2. Setup Rust
Go to the project folder and force the rustup tool to use night rust
```bash
cd Phoenix && rustup toolchain install nightly && rustup override set nightly
```
## 3. Install aditional Rust compoments required by PhoenixOS
```bash
rustup component add rust-src llvm-tools-preview && cargo kmake update
```

## Build and Run 
>If you haven't encountered any bugs along the way, you're ready to build and run Phoenix. Use the command below to build Phoenix.

```bash 
cargo kmake run --arch x86_64 
```
# Build on Windows
<details><summary>Click to expand</summary>
 

</details>

# Build on MacOS
<details><summary>Click to expand</summary>
 

</details>