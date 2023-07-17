//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

use serde::{Serialize, Deserialize};
use self::vm_slots::Slot;
use std::fs;
mod reformat;
pub(crate) mod vm_slots;

#[derive(Serialize, Deserialize)]
pub(crate) struct Args {
    pub qemu_bin: String,
    pub vnc_start_port: u16,
    pub static_files: String,
    pub vm_slots: i32
}

pub(crate) fn config() -> (Vec<String>, String, String, Vec<Slot>) {
    let config: Args = toml::from_str(fs::read_to_string("config/config.toml").expect("Should have been able to read the file").as_str()).expect("Should have been able to read the file");

    let virtual_machines = vm_slots::make(config.vnc_start_port, config.vm_slots);

    let qemu_args = reformat::split_argument(fs::read_to_string("./config/qemu.args").expect("Should have been able to read the file"));
    return (qemu_args, config.qemu_bin, config.static_files, virtual_machines);
}