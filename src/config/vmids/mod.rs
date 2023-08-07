use crate::{
    execute::VirtualMachines, config::{Config, VmidConfig}, Args, Error
};
use serde::{Deserialize, Serialize};
use std::{fs, process::Child};
pub(crate) mod addmutex;
pub(crate) mod make;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Vmid {
    pub vmid_number: usize,
    pub port: u16,
    pub qemu_arg: Vec<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub child: Option<Child>,
}


pub(crate) fn config(config: Config, args: Args) -> Result<VirtualMachines, Error> {
    let virtual_machines = if args.setup {
        VmidConfig::default()
    } else {
        serde_json::from_str(
            fs::read_to_string(config.qemu_args.clone())
                .expect("Unable to read the file: is it there?")
                .as_str(),
        ).map_err(|err| Error::ConfigError(err))?
    };

    info!("deserialized = {:#?}", virtual_machines);

    return Ok(VirtualMachines {
        virtual_machines: addmutex::run(virtual_machines.0),
        config,
        args,
    });
}