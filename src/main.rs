/*
=================================================
                xxxxxxxxxxxxx

              xxxxxxxxxxxxxxxxxxxx

     https://github.com/Toiletpaperepic/app

=================================================
*/

///todo: name it

#[macro_use]
extern crate rocket;

use crate::execute::{
    VirtualMachines, statistics, start_qemu, stop_qemu
};
use crate::embed::novnc_embed;
use crate::websocket::stream;
use rocket::fs::FileServer;
use common::test_run;
mod execute;
mod common;
mod config;
mod embed;
mod websocket;

#[launch]
fn rocket() -> _ {
    println!("Starting App_Untitled. (version: {})",env!("CARGO_PKG_VERSION"));
    let config = config::config();

    rocket::build()
        .manage(config)
        .mount("/api", routes![stream ,stop_qemu, start_qemu, statistics])
        .mount("/", routes![index, novnc_embed, frontend_embed])
}
