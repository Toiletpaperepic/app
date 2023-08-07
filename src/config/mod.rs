use crate::{execute::VirtualMachines, Args, common::test_run, Error};
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};
use self::vmids::Vmid;
pub(crate) mod vmids;

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct VmidConfig(pub Vec<Vmid>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Config {
    pub qemu_bin: PathBuf,
    pub qemu_args: PathBuf,
    pub static_files: PathBuf,
    pub stream_buffer: usize,
    pub start_port: u16
}

pub(crate) fn config(mut args: Args) -> Result<VirtualMachines, Error> {
    info!("Found {:#?}", args);

    info!("loading config...");
    let config: Config = serde_json::from_str(
        fs::read_to_string(args.config.get_or_insert(PathBuf::from("config/config.json")))
            .map_err(|err| Error::Std(err))?
            .as_str(),
    ).map_err(|err| Error::ConfigError(err))?;

    info!("Found {:#?}", config);

    test_run(config.qemu_bin.clone());

    return Ok(vmids::config(config, args)?);
}
