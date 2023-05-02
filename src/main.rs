//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/app
//
//=================================================

///todo: name it

#[macro_use] extern crate rocket;

use log::{debug, error, info, warn};
use crate::execute::VirtualMachines;
use crate::execute::start_qemu;
use rocket::fs::{relative, FileServer};
mod execute;
mod env_logger;
mod config;

#[get("/hello")]
fn hello() -> &'static str {
    //vm.start_qemu();
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    env_logger::env_logger();
    info!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
    let config = config::config();
    info!("Finished");

    rocket::build()
        .manage(VirtualMachines {
            qemu_args: config.0,
            qemu_bin: config.1,
            machine_data: config.2.into()
        })
        .mount("/api", routes![hello, start_qemu])
        .mount("/", FileServer::from(relative!("static")))
}