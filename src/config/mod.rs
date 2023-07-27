use crate::{execute::VirtualMachines, Args, setup, common::test_run};
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};
pub(crate) mod slots;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Config {
    pub qemu_bin: PathBuf,
    pub qemu_args: PathBuf,
    pub vnc_start_port: u16,
    pub vm_slots: usize,
    pub stream_buffer: usize
}

pub(crate) fn config(mut args: Args) -> VirtualMachines {
    info!("Found {:#?}", args);

    if args.setup {
        info!("=================================\nSetup is true! Moving to setup...\n=================================");
        setup::gotosetup();
    }

    info!("loading config...");
    let config: Config = toml::from_str(
        fs::read_to_string(args.config.get_or_insert(PathBuf::from("config/config.toml")))
            .expect("Unable to read the file: is it there? Maybe try --setup.")
            .as_str(),
    )
    .unwrap();

    info!("Found {:#?}", config);

    test_run(config.qemu_bin.clone());

    return slots::config(config);
}
