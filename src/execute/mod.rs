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
    unimplemented!() 
}

#[get("/stop?<slot_number>")]
pub(crate) fn stop_qemu(slot_number: i32, vms: &State<VirtualMachines>) -> Value {
    for slot in &vms.virtual_machine_data {
        if slot.slot_number == <i32 as TryInto<u16>>::try_into(slot_number).unwrap() {
            let mut used = slot.used.lock().unwrap();
            if *used {
                *used = false;
                slot.child.lock().unwrap().as_mut().unwrap().kill().unwrap();
                return json!({"status": "ok"});
            } else {
                return json!({"status": "failed"});
            }
        }
    }.into()
}

#[get("/start")]
pub(crate) fn start_qemu(vms: &State<VirtualMachines>) -> Value { 
    for slot in &vms.virtual_machine_data {
        let mut used = slot.used.lock().unwrap();
        if *used {
            info!("slot {} not Available", slot.slot_number)
        } else {
            info!("Slot {} available. starting", slot.slot_number);

            //clones and adds "-vnc :0,websocket" to the arguments
            let mut args = vms.qemu_args.clone();
            args.push("-vnc".to_owned());
            args.push(format!(":{},websocket", slot.port - 5700));
            println!("{:?}", args);
            println!("\n{}", vms.version_msg);

            let vm = Command::new(vms.qemu_bin.clone())
            .args(args)
            .spawn()
            .expect("command failed to start");

            *slot.child.lock().unwrap() = Some(vm);
            *used = true;
            
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