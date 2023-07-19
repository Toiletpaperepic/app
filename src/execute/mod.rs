use std::{path::PathBuf, process::Command, usize};
use crate::config::slots::vmid::Vmid;
use serde::Serialize;
use rocket::{
    serde::json::{
        json, Value
    },
    State, 
    http::uri::Origin,
};


pub(crate) struct VirtualMachines {
    pub qemu_args: Vec<String>,
    pub qemu_bin: PathBuf,
    pub version_msg: String,
    pub virtual_machines: Vec<Vmid>,
}

#[derive(Serialize)]
struct VmList {
    runing: bool,
    vmid: usize,
    url: String,
    stop: String,
    start: String
}

#[get("/statistics", format = "application/json")]
pub(crate) fn statistics(vms: &State<VirtualMachines>) -> Value {
    let mut count: usize = 0;
    let mut vm_list: Vec<VmList> = Vec::new();
    for vmid in &vms.virtual_machines {
        vm_list.push(VmList {
            runing: vmid.child.lock().unwrap().is_some(),
            url: format!("/noVNC/vnc.html?path=api/stream/{}",vmid.vmid_number),
            stop: format!("/api/stop?number={}",vmid.vmid_number),
            start: format!("/api/start?number={}",vmid.vmid_number),
            vmid: count
        });
        count += 1
    }
    return json!({"slot": vm_list.len(), "vm_list": vm_list});
}

#[get("/stop?<number>", format = "application/json")]
pub(crate) fn stop_qemu(number: usize, vms: &State<VirtualMachines>) -> Value {
    if vms.virtual_machines.len() > number {
        let mut vm_child = vms.virtual_machines[number].child.lock().unwrap();
        if vm_child.is_some() {
            vm_child.as_mut().unwrap().kill().unwrap();
            *vm_child = None;
            return json!({"status": "ok"});
        } else {
            return json!({"status": "Failed", "Reason": "It's not Running."});
        }
    } else {
        return json!({"Status": "Failed", "Reason": "The Requested VM Doesn't exist."});
    }
}

///Execute the virtual machine
#[get("/start?<number>", format = "application/json")]
pub(crate) fn start_qemu(number: usize, vms: &State<VirtualMachines>) -> Value {
    if vms.virtual_machines.len() > number {
        let vmid = &vms.virtual_machines[number];
        let mut vm_child = vmid.child.lock().unwrap();
        if vm_child.is_none() {
            //adds "-vnc :0,websocket" to the arguments
            let mut args = vms.qemu_args.clone();
            args.push("-vnc".to_string());
            args.push(format!(":{}", vmid.port - 5900));

            let vm = Command::new(vms.qemu_bin.clone())
                .args(args)
                .spawn()
                .expect("command failed to start");

            *vm_child = Some(vm);

            return json!({
                "status": "ok",
                "slot number": vmid.vmid_number,
                "url": format!("/noVNC/vnc.html?path=api/stream/{}", vmid.vmid_number),
                "stopurl": format!("/api/stop?number={}", vmid.vmid_number)
            });
        } else {
            return json!({"status": "Failed", "Reason": "It's already running."});
        }
    } else {
        return json!({"Status": "Failed", "Reason": "The Requested VM Doesn't exist."});
    }
}
