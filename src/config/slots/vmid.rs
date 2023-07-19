use std::{
    process::Child,
    sync::{Arc, Mutex},
};

pub(crate) struct Vmid {
    pub vmid_number: i32,
    pub port: u16,
    pub child: Arc<Mutex<Option<Child>>>,
}

pub(crate) fn make(mut qenu_port: u16, vm_slots: usize) -> Vec<Vmid> {
    let mut vec: Vec<Vmid> = Vec::new();
    let mut qenu_port_usize = <u16 as TryInto<usize>>::try_into(qenu_port).unwrap();
    let stop = qenu_port_usize + vm_slots + 1;
    let mut vmid_number = 0;

    loop {
        let vmid = Vmid {
            vmid_number,
            port: qenu_port,
            child: Arc::new(Mutex::new(None::<Child>)),
        };
        println!("in, vmid: {} port{}", vmid.vmid_number, vmid.port);
        vec.push(vmid);
        qenu_port += 1;
        qenu_port_usize += 1;
        vmid_number += 1;

        if stop == qenu_port_usize {
            break;
        }
    }
    return vec;
}
