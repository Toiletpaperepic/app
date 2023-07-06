//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

use serde::{Serialize, Deserialize};
use crate::execute::VirtualMachines;
use std::{fs, path::PathBuf};
use crate::test_run;
pub(crate) mod vm_slots;

#[derive(Serialize, Deserialize)]
pub(crate) struct Args {
    pub qemu_bin: PathBuf,
    pub vnc_start_port: u16,
    pub static_files: String,
    pub vm_slots: i32
}

pub(crate) fn config() -> VirtualMachines {
    let config: Args = toml::from_str(fs::read_to_string("config/config.toml").expect("Unable to read the file: is it there?").as_str()).unwrap();
    let qemu_args = shell_words::split(fs::read_to_string("config/qemu.args").expect("Unable to read the file: is it there?").as_str()).unwrap();
    let virtual_machines = vm_slots::make(config.vnc_start_port, config.vm_slots);

    return VirtualMachines {
        qemu_args,
        qemu_bin: config.qemu_bin.clone(),
        version_msg: test_run(config.qemu_bin.clone()).unwrap(),
        virtual_machine_data: virtual_machines,
    };
}