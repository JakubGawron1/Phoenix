<p align="center">
    <img src="./other/icon.png" style="height:300px" alt="Phoenix icon">
</p>

# Phoenix
![workflow](https://img.shields.io/github/workflow/status/JakubGawron1/Phoenix/Phoenix?label=Build)
![workflow](https://img.shields.io/github/workflow/status/JakubGawron1/Phoenix/MacOS%20build?label=Build%20on%20MacOS)
![workflow](https://img.shields.io/github/workflow/status/JakubGawron1/Phoenix/Windows%20build?label=Build%20on%20Windows)
![Lines of code](https://img.shields.io/tokei/lines/github/JakubGawron1/Phoenix?label=Total%20Lines)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/JakubGawron1/Phoenix?label=Code%20Size)

**Phoenix(OS)** is a hobby operating system for learning how to create operating systems and later developing in this field. Currently runs on two processor architectures (`X86_64` and `AArch64`) and uses its own bootloader (currently only `uefi` support).

# Features
> Here are the features I would like to have in the project. You can find the latest changes (added together with commit) in the file [CHANGELOG.md](CHANGELOG.md) 
- [x] A working builder for Phoenix (`phoenix-builder)
- [ ] Limine chainloader for X86_64 
- [ ] Print from bootloader 
- [ ] Working bootloader
- [ ] Print from kernel
# How to build
Building Phoenix is not difficult. You can build Phoenix on any operating system(`Linux`, `MacOS` and `Windows`), although it is recommended to build it on Linux. You can find more information [here]()

## Setup environment
At the very beginning, you need to install a few required things:

- Rust
- Qemu
- Make (optional)
- Python (optional)

### Rust

To install Rust on a **Unix-like** system use this command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After running the command in the terminal, follow the information from the terminal.
More information on installing on a unix-like system can be found [here](https://doc.rust-lang.org/book/ch01-01-installation.html)

To install Rust on **Windows** continue with [this tutorial](https://doc.rust-lang.org/book/ch01-01-installation.html)

### Qemu

To install `Qemu` emulator on a **Unix-like** system run this command:
```bash
sudo apt update && sudo apt upgrade && sudo apt install qemu qemu-system-x86 qemu-system-arm
```
To install `Qemu` emulator on **Windows**, download the latest version from [this page](https://qemu.weilnetz.de/w64/), run the installer, and then follow the on-screen instructions

### Make (optional)
To install `Make` on a **Unix-like** system run this command:
```bash
sudo apt update && sudo apt upgrade && sudo apt install make
```
To install `Make` on **Windows**, download the latest version from [this page](https://sourceforge.net/projects/gnuwin32/files/make/3.81/), run the installer, and then follow the on-screen instructions

### Python (optional)
To install `Python` on a **Unix-like** system run this command:
```bash
sudo apt update && sudo apt upgrade && sudo apt install python3 python3-is-python
```

To install `Python` on **Windows**, download the latest version from [this page](https://www.python.org/downloads/), run the installer, and then follow the on-screen instructions


# Contribution

# License
The project is licensed under the `Apache` license. All legal information can be found [here](LICENSE) or on https://www.apache.org/licenses/
