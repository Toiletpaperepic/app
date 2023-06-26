//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

use std::process::Command;
use crate::config::vm_slots::Slot;
use rocket::{State, serde::json::{Value, json}};

pub(crate) struct VirtualMachines {
    pub qemu_args: Vec<String>,
    pub qemu_bin: String,
    pub version_msg: String,
    pub virtual_machine_data: Vec<Slot>
}

#[get("/statistics")]
pub(crate) fn statistics(vms: &State<VirtualMachines>) -> Value {
    let mut slot_list:Vec<bool> = Vec::new();
    for slot in &vms.virtual_machine_data {
        slot_list.push(slot.child.lock().unwrap().is_some())
    }
    return json!({"slot": slot_list.len(), "slot_list": slot_list});
}

#[get("/stop?<slot_number>")]
pub(crate) fn stop_qemu(slot_number: i32, vms: &State<VirtualMachines>) -> Value {
    for slot in &vms.virtual_machine_data {
        let mut slot_child = slot.child.lock().unwrap();
        if slot_child.is_some() {
            if slot.slot_number == slot_number {
                if slot_child.is_some() {
                    slot_child.as_mut().unwrap().kill().unwrap();
                    *slot_child = None;
                    return json!({"status": "ok"});
                } else {
                    return json!({"status": "failed"});
                }
            }
        }
    }.into()
}

///Execute the virtual machine,
///needs more optimizing
#[get("/start")]
pub(crate) fn start_qemu(vms: &State<VirtualMachines>) -> Value { 
    for slot in &vms.virtual_machine_data {
        let mut slot_child = slot.child.lock().unwrap();
        if slot_child.is_some() {
            //continue;
        } else {
            info!("Slot {} available. starting", slot.slot_number);

            //clones and adds "-vnc :0,websocket" to the arguments
            let mut args = vms.qemu_args.clone();
            args.push("-vnc".to_owned());
            args.push(format!(":{},websocket", slot.port - 5700));
            println!("\n{}", vms.version_msg);

            let vm = Command::new(vms.qemu_bin.clone())
            .args(args)
            .spawn()
            .expect("command failed to start");

            *slot_child = Some(vm);
            
            return json!({
                "status": "ok",
                "slot number": slot.slot_number,
                "url": format!("/noVNC/vnc.html?path=&port={}", slot.port),
                "stopurl": format!("/api/stop?slot_number={}", slot.slot_number)
            })
        }
    }

    json!({"status": "failed", "Reason": "No slots open."})
}