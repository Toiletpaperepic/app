use log::{LevelFilter, debug, info, trace, warn};
use std::path::PathBuf;
use crate::loger::init;
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

fn main() {
    let mut args = Args::parse();
    init(*args.log_level.get_or_insert(LevelFilter::Info)).unwrap();
    trace!("Got {:#?}", args);

    trace!("hello, World");
    debug!("hello, World");
    warn!("hello, World");
    info!("hello, World");
}