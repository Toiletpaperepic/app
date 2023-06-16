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

use crate::execute::{
    VirtualMachines, statistics, start_qemu, stop_qemu
};
use crate::embed::frontend_embed;
use crate::embed::novnc_embed;
use crate::embed::index;
use common::test_run;
mod execute;
mod config;
mod common;
mod embed;

#[launch]
fn rocket() -> _ {
    println!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
    let config = config::config();
    let version_msg = test_run(config.1.clone());

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
        .mount("/", routes![index, novnc_embed, frontend_embed])
}