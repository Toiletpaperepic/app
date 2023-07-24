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

use std::path::PathBuf;

use crate::execute::{
    statistics, start_qemu, stop_qemu
};
use crate::embed::{index, novnc_embed, frontend_embed};
use crate::websocket::stream;
use common::test_run;
use clap::Parser;
mod websocket;
mod execute;
mod common;
mod config;
mod embed;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub(crate) struct Args {
    ///setup your server configuration...
    #[arg(long)]
    setup: bool,

    ///Properties file to use
    #[arg(short, long)]
    config: Option<PathBuf>,
    
    ///print version
    #[arg(short, long)]
    version: bool
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    println!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
    let config = config::config(args);

    rocket::build()
        .manage(config)
        .mount("/api", routes![stream ,stop_qemu, start_qemu, statistics])
        .mount("/", routes![index, novnc_embed, frontend_embed])
}
