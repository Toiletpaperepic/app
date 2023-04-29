//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/xxxxxxxxxxxxx
//
//=================================================

///todo: name it

#[macro_use] extern crate rocket;

use log::{debug, error, info, warn};
use std::{io::stdin, process::{self, Command}, thread};
use rocket::{fs::FileServer, Rocket, Build, fairing::AdHoc};
use crate::execute::VmsManager;
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

    info!("Finished");

    rocket::build()
        .attach(VmsManager::default())
        .mount("/", routes![hello])
        .mount("/noVNC", FileServer::from("./lib/noVNC"))
}