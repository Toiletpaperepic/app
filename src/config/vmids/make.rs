use std::{process::Child, fs::File, io::Write, path::PathBuf};
use crate::{Error, config::{VmidConfig, vmids::make}};
use super::Vmid;

pub(crate) fn vmid(qenu_port: u16, vmid_number: usize, qemu_arg: Vec<String>) -> Vmid {
    let vmid = Vmid {
        vmid_number,
        port: qenu_port,
        child: None::<Child>,
        qemu_arg
    };
    
    return vmid;
}

//only for testing
pub(crate) fn vmid_vec(mut qenu_port: u16, vmids: usize) -> Vec<Vmid> {
    let mut vec: Vec<Vmid> = Vec::new();

    for vmid_number in 0..vmids {
        let vmid = vmid(qenu_port, vmid_number, vec![]);
        info!("preloading.... {:#?}", vmid);
        vec.push(vmid);
        qenu_port += 1;
    }

    return vec;
}

pub(crate) fn config(start_port: u16, vmids: usize, vmidfile: PathBuf) -> Result<(), Error> {
    let vmids = serde_json::to_string_pretty(&VmidConfig(make::vmid_vec(start_port,vmids)))
        .map_err(|err| Error::ConfigError(err))?;

    println!("serialized = {}", vmids);

    let mut file = File::create(vmidfile).map_err(|err| Error::Std(err))?;
    file.write_all(vmids.as_bytes()).map_err(|err| Error::Std(err))?;

    let deserialized: VmidConfig = serde_json::from_str(&vmids).unwrap();
    println!("deserialized = {:#?}", deserialized);
    Ok(())
}

#[test]
fn test() {
    let vmid = make::vmid_vec(5900,4);

    assert_eq!(vmid.len(), 4);

    //test 0
    assert_eq!(vmid[0].port, 5900);
    assert_eq!(vmid[0].vmid_number, 0);

    //test 4
    assert_eq!(vmid[3].port, 5903);
    assert_eq!(vmid[3].vmid_number, 3);
}