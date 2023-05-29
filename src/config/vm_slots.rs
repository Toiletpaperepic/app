//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

use std::{sync::{Mutex, Arc}, process::Child};

pub(crate) struct Slot {
    pub used: Arc<Mutex<bool>>,
    pub slot_number: u16,
    pub port: u16,
    pub child: Arc<Mutex<Option<Child>>>
}

pub(crate) fn make(mut qenu_port: u16, vm_slots: i32) -> Vec<Slot> {
    let mut vec: Vec<Slot> = Vec::new();
    let stop = qenu_port + <i32 as TryInto<u16>>::try_into(vm_slots).unwrap();
    let mut slot_number = 1;
    
    loop {
        let slot = Slot {
            slot_number,
            port: qenu_port,
            used: Arc::new(Mutex::new(false)),
            child: Arc::new(Mutex::new(None::<Child>))
        };
        vec.push(slot);
        qenu_port += 1;
        slot_number += 1;

        if stop == qenu_port {
            break;
        }
    }
    return vec;
}
