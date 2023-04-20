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
use std::{io::stdin, process, thread};
use rocket::{fs::FileServer, Rocket, Build};
mod execute;
mod env_logger;
mod config;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    ctrlc::set_handler(move || {
        warn!("terminating on signal 2");
        process::exit(2);
    })
    .expect("Error setting Ctrl-C handler");

    env_logger::env_logger();
    info!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
    let config = config::config();

    let mut vm = execute::VmsManager::new(config.0, config.1, config.2);

    let stdin = stdin();
    let mut s_buffer = String::new();

    info!("Finished");

    rocket::build()
        .mount("/", routes![hello])
        .mount("/noVNC", FileServer::from("./lib/noVNC"))

    // loop {
    //     s_buffer.clear();
    //     stdin.read_line(&mut s_buffer).unwrap();
    //     let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");

    //     if line.starts_with("v") {
    //         thread::scope(|s| {
    //             s.spawn(|| {
    //                 vm.start_qemu()
    //             });
    //         });
    //     } else if line.starts_with("e") {
    //         info!("Exiting");
    //         break;
    //     } else {
    //         error!("Unknown command")
    //     }
    // }
}