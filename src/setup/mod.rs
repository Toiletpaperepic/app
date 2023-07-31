use std::{process, fs::File, io::{Write, self}};
use crate::config::vmids::preload::{run, Vmid};
use rocket::serde::json::serde_json;
use serde::{Serialize, Deserialize};
// mod dialog;

const START_PORT: u16 = 5900;
const VMIDS: usize = 5;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct VmidConfig (pub Vec<Vmid>);

pub(crate) fn gotosetup() -> io::Result<()> {
    let vmids = serde_json::to_string_pretty(&VmidConfig(run(START_PORT, VMIDS))).unwrap();
    println!("serialized = {}", vmids);

    let mut file = File::create("config/vmids.json")?;
    file.write_all(vmids.as_bytes())?;

    let deserialized: VmidConfig = serde_json::from_str(&vmids).unwrap();
    println!("deserialized = {:#?}", deserialized);
    // dialog::new();
    process::exit(0);
}