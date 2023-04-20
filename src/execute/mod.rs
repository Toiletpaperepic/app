//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/xxxxxxxxxxxxx
//
//=================================================

use std::process::Command;

use log::debug;

pub(crate) struct VmsManager {
    pub qemu_bin: String,
    pub vm_slots: Vec<i32>,
    pub qemu_args: Vec<String>
    //pub config_json: Args
}

impl<'a> VmsManager {
    pub fn new(qemu_args: Vec<String>, qemu_bin: String, vm_slots: Vec<i32>) -> VmsManager { 
        debug!("Running VmsManager");
        VmsManager {
            qemu_args,
            qemu_bin,
            vm_slots
            //config_json,
        }
    }

    pub fn start_qemu(&mut self) { 
        let port = self.vm_slots[0];
        self.qemu_args.push("-vnc ".to_owned());
        self.qemu_args.push(format!(":{},websocket", port - 5900));
        println!("{:?}", &self.qemu_args);

        let mut child = Command::new(&self.qemu_bin)
        .args(&self.qemu_args)
        .spawn()
        .expect("command failed to start");

        // // println!("status: {}", output.status);
        // let ecode = child.wait()
        //          .expect("failed to wait on child");

        // println!("vm stop {}", ecode.success());
        // let len = self.vm_slots.len();
        // println!("{:?}", self.vm_slots.rotate_left(len));
        // self.vm_slots
    }
}