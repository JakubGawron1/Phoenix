# CHANGELOG

##  05.01.2022 (part 1)
- Create repository
- Init commit

## 05.01.2022 (part 2)
- Created `phoenix-builder` package 
- Added cargo alias

## 05.01.2022 (part 3)
- Created CHANGELOG.md file
- Added crates to `phoenix-builder`
- Created mirror on [GitLab](https://gitlab.com/JakubGawron1/phoenix)

## 06.01.2022 (part 1)
- Created `phoenix-bootloader` package
- Created `phoenix-kernel` package
- Support for command line arguments has been added for `phoenix-builder` (all commands are available [here]())
- From now on you can build and boot the kernel and bootloader in [Qemu](https://www.qemu.org/) with `phoenix-builder`
- Fix the dates in `CHANGELOG.md (I'm still alive in 2021)

## 07.01.2022 (part 1)
- Added github workfows for `Mac os`, `Windows` and `Ubuntu`
- Better parse path to target in `phoenix-builder`
- Added badges about workflows
- Fix workflows with `rust-toolchain` file

## 07.01.2022 (part 2)
- Added license
- Added more badges
- Added Icon
- Added description and other necessary information to `README.md` 

## 15.01.2022 (part 1)
- Print to screen from `phoenix-bootloader`
- Added ASCII INTRO for `phoenix-bootloader`
- Added `splash.bmp` (will be dislayed on the screen in the future)
- Added author info to `Cargo.toml` files

## 15.01.2022 (part 2)
- Added empty `Makefile`
- Added empty `build.py` file
- Added build instructions to `README.md` (first part)
- Added empty gitpod bash file

## 20.01.2022 (part 1)
- Successfully display the image in bmp format (only X86_64) `phoenix-bootloader`
- Add colored subtitles (only X86_64) `phoenix-bootloader`
- Change the size of `splash.bmp`
- Add a special image on which the changes in BMP display were tested

## 20.01.2022 (reapir path)
- Fix `system_table` mutability
- Fix Github workflow

## 20.01.2022 (part 2)
- Add a working chainloader `Limine`
- Add the ability to select a bootloader (X86 only) `phoenix-builder`
- Remove artifacts from workflows for `MacOS` and `Windows`
- Add a new artifact to `build.yml` for build with `limine`

## 24.01.2022 (repair path)
- Fix `rust-analyzer` warnings by adding a target to `config.toml`
- Change the system image directories for `release` and `debug` profiles to make it easier to distinguish in what mode it was built
- Due to changes in saving images (their locations) fix github workflow (`build.yml`)
- Now you don't need to pass the `--bootloader` flag when building with `Phoenix Bootloader` without a `Limine` chainloader
- Make it impossible to build `AArch64 Phoenix` with `Limine`

## 24.01.2022
- Change the splash intro to show at the beginning (`phoenix-bootloader`)
- Center the text on screen (`phoenix-bootloader`) (`X86_64` architecture only)
- Add `--nographic` flag, now when you use this flag you won't see `Qemu` GUI (`phoenix-builder`)
- Termux support added
- Add `termux.sh` file to install tools on `Termux`

## 10.02.2022
- Remove splash intro, because it isn't efficient (`phoenix-bootloader`)
- Start writing bootloader with a new approach
- Add GPG key to be able to create verified commits