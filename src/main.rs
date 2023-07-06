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
use crate::embed::novnc_embed;
use rocket::fs::FileServer;
use common::test_run;
mod execute;
mod config;
mod common;
mod embed;

#[launch]
fn rocket() -> _ {
    println!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
    let config = config::config();

    rocket::build()
        .manage(config)
        // .attach(VirtualMachines::default())
        .mount("/api", routes![stop_qemu, start_qemu, statistics])
        .mount("/", FileServer::from("lib/frontend"))
        .mount("/", routes![novnc_embed])
}