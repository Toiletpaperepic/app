use std::{fs, process::Child, sync::{Mutex, Arc}, path::PathBuf};
use crate::{config::VmidConfig, Error};
use serde::{Deserialize, Serialize};
pub(crate) mod new;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Vmid {
    pub vmid_number: usize,
    pub name: String, 
    pub port: u16,
    pub qemu_arg: Vec<String>,
    pub password: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub child: Option<Child>
}

pub(super) fn config(mut vmids: Option<PathBuf>, setup: bool) -> Result<Vec<Arc<Mutex<Vmid>>>, Error> {
    let virtual_machines = if setup {
        VmidConfig::default()
    } else {
        serde_json::from_str(
            fs::read_to_string(vmids.get_or_insert(PathBuf::from("config/vmids.json")))
                .map_err(|err| Error::Std(err))?
                .as_str(),
        ).map_err(|err| Error::ConfigError(err))?
    };

    info!("deserialized = {:#?}", virtual_machines);

    return Ok(virtual_machines.1.into_iter().map(|vals| Arc::new(Mutex::new(vals))).collect::<Vec<_>>())
}