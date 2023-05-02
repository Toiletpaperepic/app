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
    pub qemu_vnc_start_port: i32,
    pub server_port: i32,
    pub vm_slots: i32,
    pub qemu_args: String
}

pub(crate) fn config() -> (Vec<std::string::String>, std::string::String, Vec<Slot>) {
    debug!("opening ./config.json");
    let config = fs::read_to_string("config/config.json").expect("Should have been able to read the file");

    // Parse the string of data into serde_json::Value.
    let config_json: Args = serde_json::from_str(&config[..]).expect("Can't Parse config.json");

    let virtual_machines = vm_slots::make(config_json.qemu_vnc_start_port, config_json.vm_slots);

    let reformated_qemu_args = reformat::split_argument(config_json.qemu_args);

    debug!("got {:?} {} {} can't show vm_slots", reformated_qemu_args, config_json.qemu_bin, config_json.qemu_vnc_start_port);

    return (reformated_qemu_args, config_json.qemu_bin, virtual_machines);
}