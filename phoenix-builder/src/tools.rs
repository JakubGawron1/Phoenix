use crate::{CARGO, bundled};

use std::process::Command;


pub fn gitpod(){}

pub fn update(){
    bundled::download_ovmf_prebuilt();
    
}