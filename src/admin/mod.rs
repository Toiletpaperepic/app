use crate::{config::vmids::make, execute::VirtualMachines};
use rocket::{futures::{StreamExt, SinkExt}, State};
use clap::{Command, arg, value_parser};
use std::path::PathBuf;
use rocket::Shutdown;

enum Commands {
    Quit,
    Message(String)
}

#[get("/console")]
pub(crate) fn console(ws: ws::WebSocket, shutdown: Shutdown, vms: &State<VirtualMachines>) -> ws::Channel<'static> {
    let vmidfile = vms.config.qemu_args.clone();
    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            match respond(message?.into_text().unwrap().as_str(), vmidfile.clone()) {
                Ok(Commands::Quit) => {
                    let _ = stream.send("Server is Going down!".into()).await;
                    info!("Server is Going down!");
                    shutdown.clone().notify();
                }
                Ok(Commands::Message(message)) => {
                    let _ = stream.send(message.clone().into()).await;
                    info!("{}", message)
                }
                Err(err) => {
                    info!("{}", err);
                    let _ = stream.send(err.into()).await;
                }
            }
        }
        Ok(())
    }))
}

fn respond(line: &str, vmidfile: PathBuf) -> Result<Commands, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("setup", matches)) => {
            make::config(*matches.get_one::<u16>("port").ok_or(0).map_err(|e| e.to_string())?, *matches.get_one::<usize>("vmids").ok_or(0).map_err(|e| e.to_string())?, vmidfile).map_err(|e| e.to_string())?;
            return Ok(Commands::Message(format!("done! ({:#?}, {:#?})", matches.get_one::<u16>("port"), matches.get_one::<usize>("vmids"))));
        }
        Some(("ping", _matches)) => {
            return Ok(Commands::Message("Pong".to_string()));
        }
        Some(("quit", _matches)) => {
            return Ok(Commands::Quit);
        }
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }
}

fn cli() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("Manual Setup")
        .subcommand_help_heading("Manual Setup")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("setup")
                    .about("Setup your Virtual Machines.")
                    .help_template(APPLET_TEMPLATE)
                    .args([
                        arg!(port: -p --port <PORT> "Starting Port.")
                            .required(true)
                            .value_parser(value_parser!(u16)),
                        arg!(vmids: -v --vmids <PORT> "how many Virtual Machines you want.")
                            .required(true)
                            .value_parser(value_parser!(usize))
                    ]),
        )
        .subcommand(
            Command::new("ping")
                    .about("Get a response")
                    .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                    .alias("exit")
                    .about("Quit the REPL")
                    .help_template(APPLET_TEMPLATE),
        )
}
