/*
=================================================
                xxxxxxxxxxxxx

              xxxxxxxxxxxxxxxxxxxx

     https://github.com/Toiletpaperepic/app

=================================================
*/

use crate::config::slots::vm_slots::Slot;
use std::{path::PathBuf, process::Command, usize};
use rocket::{
    serde::json::{json, Value},
    State,
};

pub(crate) struct VirtualMachines {
    pub qemu_args: Vec<String>,
    pub qemu_bin: PathBuf,
    pub version_msg: String,
    pub virtual_machines: Vec<Slot>,
}

#[get("/statistics", format = "application/json")]
pub(crate) fn statistics(vms: &State<VirtualMachines>) -> Value {
    let mut slot_list: Vec<bool> = Vec::new();
    for slot in &vms.virtual_machines {
        slot_list.push(slot.child.lock().unwrap().is_some())
    }
    return json!({"slot": slot_list.len(), "slot_list": slot_list});
}

#[get("/stop?<number>", format = "application/json")]
pub(crate) fn stop_qemu(number: usize, vms: &State<VirtualMachines>) -> Value {
    if vms.virtual_machines.len() > number {
        let mut slot_child = vms.virtual_machines[number].child.lock().unwrap();
        if slot_child.is_some() {
            slot_child.as_mut().unwrap().kill().unwrap();
            *slot_child = None;
            return json!({"status": "ok"});
        } else {
            return json!({"status": "Failed", "Reason": "It's not Running."});
        }
    } else {
        return json!({"Status": "Failed", "Reason": "The Requested Slot Doesn't exist."});
    }
}

///Execute the virtual machine
#[get("/start?<number>", format = "application/json")]
pub(crate) fn start_qemu(number: usize, vms: &State<VirtualMachines>) -> Value {
    if vms.virtual_machines.len() > number {
        let slot = &vms.virtual_machines[number];
        let mut slot_child = slot.child.lock().unwrap();
        if slot_child.is_none() {
            //adds "-vnc :0,websocket" to the arguments
            let mut args = vms.qemu_args.clone();
            args.push("-vnc".to_string());
            args.push(format!(":{},websocket", slot.port - 5700));

            let vm = Command::new(vms.qemu_bin.clone())
                .args(args)
                .spawn()
                .expect("command failed to start");

            *slot_child = Some(vm);

            return json!({
                "status": "ok",
                "slot number": slot.slot_number,
                "url": format!("/noVNC/vnc.html?path=&port={}", slot.port),
                "stopurl": format!("/api/stop?number={}", slot.slot_number)
            });
        } else {
            return json!({"status": "Failed", "Reason": "It's already running."});
        }
    } else {
        return json!({"Status": "Failed", "Reason": "The Requested Slot Doesn't exist."});
    }
}
