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
use rocket::{fairing::{Fairing, Info, self, Kind}, Rocket, Build, State};

use crate::config;

#[derive(Default, Clone)]
pub(crate) struct VmsManager {
    pub qemu_args: Vec<String>,
    pub qemu_bin: String,
    pub vm_slots: Vec<i32>
}

#[rocket::async_trait]
impl Fairing for VmsManager {
    fn info(&self) -> Info {
        Info {
            name: "VmsManager",
            kind: Kind::Ignite | Kind::Request
        }
    }

    async fn on_ignite(&mut self, rocket: Rocket<Build>) -> fairing::Result { 
        debug!("Running VmsManager");
        let config = config::config();

        self.qemu_args = config.0;
        self.qemu_bin = config.1;
        self.vm_slots = config.2;

        #[get("/start_qemu")]
        fn start_qemu(vms: &State<VmsManager>) -> String { 
            let port = vms.vm_slots[0];
            let mut args = vms.qemu_args.clone();
            args.push("-vnc ".to_owned());
            args.push(format!(":{},websocket", port - 5900));
            println!("{:?}", args);
    
            Command::new(vms.qemu_bin.clone())
            .args(args)
            .spawn()
            .expect("command failed to start");

            "ok".to_string()
        }

        Ok(rocket.manage(self.clone()).mount("/", routes![start_qemu]))
    }
}