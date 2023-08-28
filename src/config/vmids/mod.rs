use std::{process::Child, sync::{Mutex, Arc}, path::PathBuf};
use crate::{Error, pool::load_pool};
use serde::{Deserialize, Serialize};
pub(crate) mod new;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Vmid {
    pub vmid_number: usize,
    pub name: String, 
    pub port: u16,
    pub qemu_arg: Vec<String>,
    pub password: Option<String>,
    pub path: Option<PathBuf>,

    #[serde(skip_serializing, skip_deserializing)]
    pub child: Option<Child>
}

pub(super) fn config(mut pool: Option<PathBuf>, setup: bool) -> Result<Vec<Arc<Mutex<Vmid>>>, Error> {
    let virtual_machines = if setup {
        Default::default()
    } else {
        load_pool(pool.get_or_insert(PathBuf::from("./pool")).to_path_buf())?
    };

    // info!("deserialized = {:#?}", virtual_machines);

    Ok(virtual_machines.into_iter().map(|vals| Arc::new(Mutex::new(vals))).collect::<Vec<_>>())
}