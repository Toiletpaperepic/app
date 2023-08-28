#[macro_use] extern crate log;

use std::{path::PathBuf, fs, io::Write};
use rust_embed::RustEmbed;
use crate::loger::init;
use log::LevelFilter;
use clap::Parser;
mod loger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    ///The default directory
    #[arg(short, long)]
    dir: Option<PathBuf>,

    ///set the log level.
    #[arg(short, long)]
    log_level: Option<LevelFilter>
}

#[derive(RustEmbed)]
#[folder = "$EMDED"]
struct Asset;

fn main() {
    let args = Args::parse();
    init(args.log_level.unwrap_or_else(|| LevelFilter::Info)).unwrap();
    let dir = args.dir.clone().unwrap_or_else(|| PathBuf::from("./dir"));
    trace!("Got {:#?}", args);
    trace!("build Embed {}", env!("EMDED"));

    let package = Asset::iter().collect::<Vec<_>>();

    for file in package {
        info!("{:?}", file);

        if let Some(data) = Asset::get(&file) {
            let dir_join = dir.join(file.to_string());
            let dir_parent = dir_join.parent().unwrap();
            debug!("{}", dir_parent.display().to_string());
            
            if dir_parent.exists() {
                debug!("skiping")
            } else {
                fs::create_dir_all(dir_parent).unwrap();
            }

            fs::File::create(dir_join).unwrap().write_all(&data.data).unwrap();
            // data.data
        }
    }
}