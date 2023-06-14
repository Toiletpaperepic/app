//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

use std::{sync::{Mutex, Arc}, process::Child, io::{self, Error, ErrorKind}};

pub(crate) struct Slot {
    pub slot_number: i32,
    pub port: u16,
    pub child: Arc<Mutex<Option<Child>>>
}

impl Slot {
    pub fn is_used(&self) -> io::Result<bool> {
        let mut child_lock = self.child.lock().unwrap();

        if child_lock.is_none() {
            return Ok(false);
        }

        return match child_lock.as_mut().unwrap().try_wait() {
            Ok(Some(..)) => Ok(true),
            Ok(None) => {
                Ok(false)
            }
            Err(e) => Err(Error::new(ErrorKind::Other, format!("error attempting to wait: {e}").as_str())),
        };
    }
}

pub(crate) fn make(mut qenu_port: u16, vm_slots: i32) -> Vec<Slot> {
    let mut vec: Vec<Slot> = Vec::new();
    let stop = qenu_port + <i32 as TryInto<u16>>::try_into(vm_slots).unwrap();
    let mut slot_number = 1;
    
    loop {
        let slot = Slot {
            slot_number,
            port: qenu_port,
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
