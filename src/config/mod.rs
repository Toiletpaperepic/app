/*
=================================================
                xxxxxxxxxxxxx

              xxxxxxxxxxxxxxxxxxxx

     https://github.com/Toiletpaperepic/app

=================================================
*/

use crate::execute::VirtualMachines;
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};
pub(crate) mod slots;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Config {
    pub qemu_bin: PathBuf,
    pub vnc_start_port: u16,
    pub vm_slots: usize,
}

pub(crate) fn config() -> VirtualMachines{
    println!("loading config...");
    let config: Config = toml::from_str(
        fs::read_to_string("config/config.toml")
            .expect("Unable to read the file: is it there?")
            .as_str(),
    )
    .unwrap();

    println!("Got {:?}", config);

    return slots::config(config.clone());
}
