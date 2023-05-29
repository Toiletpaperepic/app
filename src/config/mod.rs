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
use log::debug;
use std::fs;
mod reformat;
pub(crate) mod vm_slots;

#[derive(Serialize, Deserialize)]
pub(crate) struct Args {
    pub qemu_bin: String,
    pub vnc_start_port: u16,
    pub static_files: String,
    pub server_port: u16,
    pub vm_slots: i32
}

pub(crate) fn config() -> (Vec<String>, String, String, Vec<Slot>) {
    debug!("opening ./config.json");
    let config = fs::read_to_string("config/config.json").expect("Should have been able to read the file");

    // Parse the string of data into serde_json::Value.
    let config_json: Args = serde_json::from_str(&config[..]).expect("Can't Parse config.json");

    let virtual_machines = vm_slots::make(config_json.vnc_start_port, config_json.vm_slots);

    let qemu_args = reformat::split_argument(fs::read_to_string("./config/qemu_args").expect("Should have been able to read the file"));

    debug!("got {:?} {} {} can't show vm_slots", qemu_args, config_json.qemu_bin, config_json.vnc_start_port);

    return (qemu_args, config_json.qemu_bin, config_json.static_files, virtual_machines);
}