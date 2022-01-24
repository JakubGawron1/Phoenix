use crate::{CARGO, bundled};

use std::process::Command;


pub fn gitpod(){
    let mut gitpod_command = Command::new("bash");

    gitpod_command.current_dir("tools");

    gitpod_command.arg("gitpod.sh");

    if !gitpod_command
    .status()
    .expect(&format!("Failde to run {:#?}", gitpod_command))
    .success()
    {
        panic!("Failed to setup GitPod");

    }
}

pub fn termux(){
    let mut termux_command = Command::new("bash");

    termux_command.current_dir("tools");

    termux_command.arg("gitpod.sh");

    if !termux_command
    .status()
    .expect(&format!("Failde to run {:#?}", termux_command))
    .success()
    {
        panic!("Failed to setup Termux");

    }
}
