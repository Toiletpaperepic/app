//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

use std::sync::{Mutex, Arc};

pub(crate) struct Slot {
    pub url: String,
    pub used: Arc<Mutex<bool>>,
    pub slot_number: i32,
    pub port: i32
}

pub(crate) fn make(mut qenu_port: i32, vm_slots: i32) -> Vec<Slot> {
    let mut vec: Vec<Slot> = Vec::new();
    let stop = qenu_port + vm_slots;
    let mut slot_number = 0;
    
    loop {
        let slot = Slot {
            slot_number,
            port: qenu_port,
            url: format!("/noVNC/vnc.html?path=&port={}", qenu_port),
            used: Arc::new(Mutex::new(false)),
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
