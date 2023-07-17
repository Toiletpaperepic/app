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

use crate::embed::novnc_embed;
use crate::execute::{start_qemu, statistics, stop_qemu};
use rocket::fs::FileServer;
use common::test_run;
mod execute;
mod common;
mod config;
mod embed;

#[launch]
fn rocket() -> _ {
    println!("Starting App_Untitled. (version: {})",env!("CARGO_PKG_VERSION"));
    let config = config::config();

    rocket::build()
        .manage(config)
        .mount("/api", routes![stop_qemu, start_qemu, statistics])
        .mount("/", FileServer::from("lib/frontend"))
        .mount("/", routes![novnc_embed])
}
