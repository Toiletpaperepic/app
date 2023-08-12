use crate::{
    execute::VirtualMachines, 
    config::{
        Config, VmidConfig
    }, 
    Error, 
    Args
};
use std::{fs, process::Child, sync::{Mutex, Arc}, path::PathBuf};
use serde::{Deserialize, Serialize};
pub(crate) mod new;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Vmid {
    pub vmid_number: usize,
    pub name: String, 
    pub port: u16,
    pub qemu_arg: Vec<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub child: Option<Child>
}

pub(crate) fn addmutex(vec_vmid: Vec<Vmid>) -> Vec<Arc<Mutex<Vmid>>> {
    let mut vec: Vec<Arc<Mutex<Vmid>>> = Vec::new();
    for vmid in vec_vmid {
        vec.push(Arc::new(Mutex::new(vmid)))
    }
    return vec;
}

pub(super) fn config(config: Config, args: Args) -> Result<VirtualMachines, Error> {
    let setup = args.setup;
    let virtual_machines = if setup.clone() {
        VmidConfig::default()
    } else {
        serde_json::from_str(
            fs::read_to_string(config.vmids.clone().get_or_insert(PathBuf::from("config/vmids.json")))
                .map_err(|err| Error::Std(err))?
                .as_str(),
        ).map_err(|err| Error::ConfigError(err))?
    };

    info!("deserialized = {:#?}", virtual_machines);

    return Ok(VirtualMachines {
        virtual_machines: addmutex(virtual_machines.0),
        config,
        setup
    });
}