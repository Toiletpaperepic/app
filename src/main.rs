/*
=================================================
                xxxxxxxxxxxxx

              xxxxxxxxxxxxxxxxxxxx

     https://github.com/Toiletpaperepic/app

=================================================
*/

///todo: name it

#[macro_use] extern crate rocket;

use crate::{
    execute::{
        statistics, start_qemu, stop_qemu
    },
    common::{
        favicon, index, apiserviceunavailable
    },
    websocket::{
        setup::console,
        stream::stream
    }
};
use rocket::{fairing::AdHoc, fs::FileServer, response::content::RawHtml};
use std::{path::PathBuf, io};
use clap::Parser;
mod websocket;
mod execute;
mod common;
mod config;
mod pool;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    ConfigError(serde_json::Error),
    Std(std::io::Error),
    Io(io::Error),
    Unknown
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    ///setup your server configuration,
    ///this will open Manual Setup,
    ///Guided Setup is not available.
    #[arg(long)]
    setup: bool,

    ///Config file to use. 
    ///Default file "config\config.json"
    #[arg(short, long)]
    config: Option<PathBuf>
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();

    rocket::build()
        .attach(AdHoc::on_ignite("startup", move |rocket| Box::pin(async move {
            info!("Starting Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
            let setup = args.setup;
            let vms = config::config(args).unwrap();
            let static_files = vms.config.static_files.clone().display().to_string();

            if setup {
                rocket
                    .manage(vms)
                    .mount("/home", FileServer::from(format!("{}/frontend", static_files)))
                    .mount("/setup", FileServer::from(format!("{}/setup", static_files)))
                    .mount("/api", routes![console, apiserviceunavailable])
            } else {
                #[get("/<response>")]
                fn admin(response: Option<&str>) -> RawHtml<String> {
                    RawHtml(format!("<h1>This page is disable. '{}' Does not exist.</h1>", response.unwrap_or_default()).to_string())
                }
                rocket
                    .manage(vms)
                    .mount("/home", FileServer::from(format!("{}/frontend", static_files)))
                    .mount("/noVNC", FileServer::from(format!("{}/noVNC", static_files)))
                    .mount("/api", routes![stream, stop_qemu, start_qemu, statistics])
                    .mount("/admin", routes![admin])
            }
        })))
        .mount("/", routes![index, favicon])
}