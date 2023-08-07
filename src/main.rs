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
    websocket::stream,
    admin::console,
    common::{
        favicon, index
    }
};
use rocket::{fairing::AdHoc, fs::FileServer, response::content::RawHtml};
use std::{path::PathBuf, fmt};
use clap::Parser;
mod websocket;
mod version;
mod execute;
mod common;
mod config;
mod admin;

#[derive(Debug)]
pub enum Error {
    ConfigError(serde_json::Error),
    Std(std::io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub(crate) struct Args {
    ///setup your server configuration...
    #[arg(long)]
    setup: bool,

    ///Properties file to use
    #[arg(short, long)]
    config: Option<PathBuf>,
    
    ///Print version
    #[arg(short, long)]
    version: bool
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    let setup = args.setup.clone();
    if args.version {
        version::version()
    }

    rocket::build()
        .attach(AdHoc::on_ignite("startup", move |rocket| Box::pin(async move {
            info!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
            let vms = config::config(args).unwrap();
            let static_files = vms.config.static_files.clone().display().to_string();

            if setup {
                rocket
                    .manage(vms)
                    .mount("/home", FileServer::from(format!("{}/frontend", static_files)))
                    .mount("/noVNC", FileServer::from(format!("{}/noVNC", static_files)))
                    .mount("/admin", FileServer::from(format!("{}/admin", static_files)))
                    .mount("/api", routes![console])
            } else {
                #[get("/<response>")]
                fn admin(response: Option<&str>) -> RawHtml<String> {
                    RawHtml(format!("<h1>This page is disable. '{}' Does not exist.</h1>", response.unwrap_or_default()).to_string())
                }
                rocket
                    .manage(vms)
                    .mount("/home", FileServer::from(format!("{}/frontend", static_files)))
                    .mount("/noVNC", FileServer::from(format!("{}/noVNC", static_files)))
                    .mount("/admin", routes![admin])
            }
        })))
        .mount("/api", routes![stream ,stop_qemu, start_qemu, statistics])
        .mount("/", routes![index, favicon])
}