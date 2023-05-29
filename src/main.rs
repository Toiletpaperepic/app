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

use rocket::{
    fs::{relative, FileServer}
};
use crate::execute::{
    VirtualMachines, statistics, start_qemu, stop_qemu
};
use common::test_run;
use log::info;
mod env_logger;
mod execute;
mod config;
mod common;

#[launch]
fn rocket() -> _ {
    env_logger::env_logger();
    info!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
    let config = config::config();
    let version_msg = test_run(config.1.clone());
    info!("launching Rocket");

    // let config = Config {
    //     port: 7777,
    //     address: Ipv4Addr::new(18, 127, 0, 1).into(),
    //     temp_dir: "/tmp/config-example".into(),
    //     ..Config::debug_default()
    // };

    rocket::build()
        .manage(VirtualMachines {
            qemu_args: config.0,
            qemu_bin: config.1,
            virtual_machine_data: config.3.into(),
            version_msg : version_msg.unwrap()
        })
        .mount("/api", routes![stop_qemu, start_qemu, statistics])
        .mount("/", FileServer::from(config.2))
}