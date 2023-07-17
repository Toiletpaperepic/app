/*
=================================================
                xxxxxxxxxxxxx

              xxxxxxxxxxxxxxxxxxxx

     https://github.com/Toiletpaperepic/app

=================================================
*/

use std::{
    process::Child,
    sync::{Arc, Mutex},
};

pub(crate) struct Slot {
    pub slot_number: i32,
    pub port: u16,
    pub child: Arc<Mutex<Option<Child>>>,
}

pub(crate) fn make(qenu_port: u16, vm_slots: usize) -> Vec<Slot> {
    let mut vec: Vec<Slot> = Vec::new();
    let mut qenu_port_usize = <u16 as TryInto<usize>>::try_into(qenu_port).unwrap();
    let stop = qenu_port_usize + vm_slots;
    let mut slot_number = 0;

    loop {
        let slot = Slot {
            slot_number,
            port: qenu_port,
            child: Arc::new(Mutex::new(None::<Child>)),
        };
        vec.push(slot);
        qenu_port_usize += 1;
        slot_number += 1;

        if stop == qenu_port_usize {
            break;
        }
    }
    return vec;
}
