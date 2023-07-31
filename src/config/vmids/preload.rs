use serde::{Serialize, Deserialize};
use std::process::Child;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Vmid {
    pub vmid_number: i32,
    pub port: u16,
    pub qemu_arg: Vec<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub child: Option<Child>,
}

pub(crate) fn run(mut qenu_port: u16, vm_slots: usize) -> Vec<Vmid> {
    let mut vec: Vec<Vmid> = Vec::new();
    let mut qenu_port_usize = <u16 as TryInto<usize>>::try_into(qenu_port).unwrap();
    let stop = qenu_port_usize + vm_slots;
    let mut vmid_number = 0;

    loop {
        let vmid = Vmid {
            vmid_number,
            port: qenu_port,
            child: None::<Child>,
            qemu_arg: vec![],
        };
        info!("preloading.... {:#?}", vmid);
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

#[test]
fn test() {
    let vmid = run(5900,4);

    assert_eq!(vmid.len(), 4);

    //test 0
    assert_eq!(vmid[0].port, 5900);
    assert_eq!(vmid[0].vmid_number, 0);

    //test 4
    assert_eq!(vmid[3].port, 5903);
    assert_eq!(vmid[3].vmid_number, 3);
}