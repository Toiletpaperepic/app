/*
=================================================
                xxxxxxxxxxxxx

              xxxxxxxxxxxxxxxxxxxx

     https://github.com/Toiletpaperepic/app

=================================================
*/

///todo: name it

#[macro_use] extern crate rocket;

use crate::execute::{
    statistics, start_qemu, stop_qemu
};
use crate::embed::{index, novnc_embed, frontend_embed};
use crate::websocket::stream;
use rocket::fairing::AdHoc;
use std::path::PathBuf;
use clap::Parser;
mod websocket;
mod version;
mod execute;
mod common;
mod config;
mod embed;
mod setup;

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
    if args.version {
        version::version()
    } else if args.setup {
        println!("Moving to setup...");
        setup::gotosetup().unwrap();
    }

    rocket::build()
        .attach(AdHoc::on_ignite("startup", |rocket| Box::pin(async {
            info!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
            let config = config::config(args);

            rocket.manage(config)
        })))
        .mount("/api", routes![stream ,stop_qemu, start_qemu, statistics])
        .mount("/", routes![index, novnc_embed, frontend_embed])
}
