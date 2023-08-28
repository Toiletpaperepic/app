use crate::{execute::VirtualMachines, Args, common::test_run, Error};
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};
pub(crate) mod vmids;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Config {
    pub qemu_bin: Option<PathBuf>,
    pub pool: Option<PathBuf>,
    pub static_files: PathBuf,
    pub stream_buffer: Option<usize>,
    pub default_args: Option<Vec<String>>,
    pub start_port: u16
}

pub(crate) fn config(mut args: Args) -> Result<VirtualMachines, Error> {
    info!("Found {:#?}", args);

    info!("loading config...");
    let config: Config = serde_json::from_str(
        fs::read_to_string(args.config.get_or_insert(PathBuf::from("config/config.json")))
            .expect("Unable to read the file: is it there? Maybe try --setup.")
            .as_str(),
    ).map_err(Error::ConfigError)?;

    info!("Found {:#?}", config);

    test_run(config.qemu_bin.clone().get_or_insert(PathBuf::from("qemu-system-x86_64")).to_path_buf());

    Ok(VirtualMachines { 
        virtual_machines: vmids::config(config.pool.clone(), args.setup)?, 
        config, 
        setup: args.setup 
    })
}
