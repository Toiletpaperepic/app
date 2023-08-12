use crate::{config::{vmids::{new, Vmid}, VmidConfig}, execute::VirtualMachines};
use rocket::{futures::{StreamExt, SinkExt}, State};
use std::{path::PathBuf, fs::File, io::Write};
use clap::{Command, arg, value_parser};
use rocket::Shutdown;

enum Commands {
    Quit,
    Vmid(Vmid),
    Message(String)
}

#[get("/console")]
pub(crate) fn console(ws: ws::WebSocket, shutdown: Shutdown, vms: &State<VirtualMachines>) -> ws::Channel<'static> {
    let vmidfile = vms.config.vmids.clone();
    let mut vmids: Vec<Vmid> = Vec::new();

    ws.channel(move |mut channel| Box::pin(async move {
        let _ = channel.send("link standby...".into()).await;
        while let Some(message) = channel.next().await {
            let _ = channel.send("link standby...".into()).await;
            let message = message?;
            if message.is_empty() | message.is_close() {
                break;
            }

            match respond(message.into_text()?.as_str(), vmidfile.clone().get_or_insert(PathBuf::from("config/vmids.json")).to_path_buf()) {
                Ok(Commands::Quit) => {
                    let _ = channel.send("Received 'EXIT': Server is Going down!".into()).await;
                    info!("Received 'EXIT': Server is Going down!");
                    shutdown.clone().notify();
                    channel.close(None).await?;
                }
                Ok(Commands::Vmid(vmid)) => {
                    vmids.push(vmid);
                    let _ = channel.send(format!("done {:#?}!", vmids).into()).await;
                    info!("done {:#?}!", vmids);
                }
                Ok(Commands::Message(message)) => {
                    let _ = channel.send(message.clone().into()).await;
                    info!("{}", message)
                }
                Err(err) => {
                    info!("{}", err);
                    let _ = channel.send(err.into()).await;
                }
            }
            let _ = channel.send("link standby...".into()).await;
        }
        Ok(())
    }))
}

fn respond(line: &str, vmidfile: PathBuf) -> Result<Commands, String> {
    info!("{}", line);
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    debug!("{:#?}", args);
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("quicksetup", matches)) => {
            let port = matches.get_one::<u16>("port").ok_or(0).map_err(|err| err.to_string())?;
            let vmids = matches.get_one::<usize>("vmids").ok_or(0).map_err(|err| err.to_string())?;

            let vmids = serde_json::to_string_pretty(&VmidConfig(new::vmid(*port, *vmids)))
                .map_err(|e| e.to_string())?;

            let mut file = File::create(vmidfile).map_err(|e| e.to_string())?;
            file.write_all(vmids.as_bytes()).map_err(|e| e.to_string())?;

            return Ok(Commands::Message(format!("done! {}", vmids)));
        }
        Some(("new", matches)) | Some(("init", matches)) => {
            let port = matches.get_one::<u16>("port").get_or_insert(&0).clone();
            let vmid = matches.get_one::<usize>("vmid").get_or_insert(&0).clone();
            let name = matches.get_one::<String>("name").get_or_insert(&"No Name".to_string()).clone();
            let args = matches.get_many::<String>("args").map(|vals| vals.collect::<Vec<_>>()).ok_or("nut".to_string())?;
            let mut qemu_arg = Vec::new();

            for arg in args {
                qemu_arg.push(arg.to_owned());
            }

            let vm = Vmid {
                vmid_number: vmid,
                name,
                port,
                qemu_arg,
                child: None,
            };

            return Ok(Commands::Vmid(vm));
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
        .subcommands([
            Command::new("quicksetup")
                .about("Setup an Array of Virtual Machines. only for testing.")
                .help_template(APPLET_TEMPLATE)
                .args([
                    arg!(port: -p --port <PORT> "Starting Port.")
                        .required(true)
                        .value_parser(value_parser!(u16)),
                    arg!(vmids: -v --vmids <PORT> "how many Virtual Machines you want.")
                        .required(true)
                        .value_parser(value_parser!(usize))
                ]),
            Command::new("new")
                .alias("init")
                .about("make a new Virtual Machines.")
                .help_template(APPLET_TEMPLATE)
                .args([
                    arg!(port: -p --port <PORT> "Starting Port.")
                        .required(false)
                        .value_parser(value_parser!(u16)),
                    arg!(vmid: -v --vmid <VMID> "Your Virtual Machines VMID.")
                        .required(true)
                        .value_parser(value_parser!(usize)),
                    arg!(name: -n --name <NAME> "The name for your Virtual Machines.")
                        .required(false)
                        .value_parser(value_parser!(String)),
                    arg!(args: [ARGS] "the command line arguments for qemu.")
                        .num_args(1..)
                        .required(true)
                        .value_parser(value_parser!(String))
                ]),
            Command::new("save")
                .help_template(APPLET_TEMPLATE),
            Command::new("ping")
                .about("Get a response")
                .help_template(APPLET_TEMPLATE),
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
            ]
        )
}
