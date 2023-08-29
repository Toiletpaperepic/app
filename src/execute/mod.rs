use crate::{config::{vmids::Vmid, Config}, websocket::stream::Destination};
use std::{process::Command, usize, sync::{Arc, Mutex}, path::PathBuf};
use serde::Serialize;
use rocket::{
    serde::json::{
        json, Value
    },
    State
};

pub(crate) struct VirtualMachines {
    pub virtual_machines: Vec<Arc<Mutex<Vmid>>>,
    pub config: Config,
    pub setup: bool
}

#[derive(Serialize)]
struct VmList {
    runing: bool,
    name: String,
    vmid: usize
}

#[get("/statistics", format = "application/json")]
pub(crate) fn statistics(vms: &State<VirtualMachines>) -> Value {
    let mut vm_list: Vec<VmList> = Vec::new();
    for vmid in &vms.virtual_machines {
        let vmid_lock = vmid.lock().unwrap();
        vm_list.push(VmList {
            runing: vmid_lock.child.is_some(),
            name: vmid_lock.name.clone(),
            vmid: vmid_lock.vmid_number
        });
    }
    json!({"vm_list": vm_list})
}

#[get("/stop?<number>", format = "application/json")]
pub(crate) fn stop_qemu(number: usize, vms: &State<VirtualMachines>) -> Value {
    if vms.virtual_machines.len() > number {
        let vmid_lock = &mut vms.virtual_machines[number].lock().unwrap();
        if vmid_lock.child.is_some() {
            vmid_lock.child.as_mut().unwrap().kill().unwrap();
            vmid_lock.child = None;
            json!({"status": "ok"})
        } else {
            json!({"status": "Failed", "Reason": "It's not Running."})
        }
    } else {
        json!({"Status": "Failed", "Reason": "The Requested VM Doesn't exist."})
    }
}

///Execute the virtual machine
#[get("/start?<number>", format = "application/json")]
pub(crate) fn start_qemu(number: usize, vms: &State<VirtualMachines>) -> Value {
    if vms.virtual_machines.len() > number {
        let vmid_lock = &mut vms.virtual_machines[number].lock().unwrap();
        if vmid_lock.child.is_none() {
            //adds "-vnc :0,websocket" to the arguments
            let mut args = vmid_lock.qemu_arg.clone();
            args.push("-vnc".to_string());
            match &vmid_lock.destination {
                #[cfg(unix)]
                Destination::Unix(path) => args.push(format!("unix:{}", path.display().to_string())),
                Destination::Tcp(port) => args.push(format!(":{}", port.port()))
            }
            args.append(&mut vms.config.default_args.clone().unwrap_or_default());
            info!("{:#?}", args);

            let vm = Command::new(vms.config.qemu_bin.clone().get_or_insert(PathBuf::from("qemu-system-x86_64")))
                .args(&args)
                .current_dir(vmid_lock.path.clone().unwrap_or_else(|| PathBuf::from("./")))
                .spawn()
                .expect("command failed to start");

            vmid_lock.child = Some(vm);

            json!({
                "status": "ok",
                "vmid": vmid_lock.vmid_number
            })
        } else {
            json!({"status": "Failed", "Reason": "It's already running."})
        }
    } else {
        json!({"Status": "Failed", "Reason": "The Requested VM Doesn't exist."})
    }
}