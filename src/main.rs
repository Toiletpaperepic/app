//=================================================
//                 xxxxxxxxxxxxx
//
//               xxxxxxxxxxxxxxxxxxxx
//
//https://github.com/Toiletpaperepic/xxxxxxxxxxxxx
//
//=================================================

///todo: name it

use clap::Parser;
use log::{debug, error, info, warn};
use std::{io::stdin, process, thread};
mod execute;
mod env_logger;
mod config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Args {
}

fn main() {
    ctrlc::set_handler(move || {
        warn!("terminating on signal 2");
        process::exit(2);
    })
    .expect("Error setting Ctrl-C handler");

    let args = Args::parse();
    env_logger::env_logger();
    info!("Starting App_Untitled. (version: {})", env!("CARGO_PKG_VERSION"));
    let config = config::config();

    let mut vm = execute::VmsManager::new(config.0, config.1, config.2, config.3, config.4, config.5);

    vm.start_novnc();

    let stdin = stdin();
    let mut s_buffer = String::new();

    info!("Finished");

    loop {
        s_buffer.clear();
        stdin.read_line(&mut s_buffer).unwrap();
        let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");

        if line.starts_with("v") {
            thread::scope(|s| {
                s.spawn(|| {
                    vm.start_qemu()
                });
            });
        } else if line.starts_with("e") {
            info!("Exiting");
            break;
        } else {
            error!("Unknown command")
        }
    }
}
