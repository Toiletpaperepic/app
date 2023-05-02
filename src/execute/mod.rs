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
    pub machine_data: Vec<Slot>
}

#[get("/start_qemu")]
pub(crate) fn start_qemu(vms: &State<VirtualMachines>) -> Value { 
    for slot in &vms.machine_data {
        let mut used = slot.used.lock().unwrap();
        if *used {
            //
        } else {
            let mut args = vms.qemu_args.clone();
            args.push("-vnc".to_owned());
            args.push(format!(":{},websocket", slot.port - 5700));
            println!("{:?}", args);

            let vm = Command::new(vms.qemu_bin.clone())
            .args(args)
            .spawn()
            .expect("command failed to start");

            *used = true;
            return json!({"status": "ok", "slot number": slot.slot_number, "url": format!("/noVNC/vnc.html?path=&port={}", slot.port)})
        }
    }

    json!({"status": "failed", "Reason": "No slots open."})
}