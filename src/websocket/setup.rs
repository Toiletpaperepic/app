use crate::{config::vmids::{new, Vmid}, pool::new_pool, execute::VirtualMachines};
use rocket::{futures::{StreamExt, SinkExt}, State};
use clap::{Command, arg, value_parser};
use std::path::PathBuf;
use rocket::Shutdown;

enum Commands {
    Quit,
    Vmid(Vmid),
    Message(String)
}

#[get("/console")]
pub(crate) fn console(ws: ws::WebSocket, shutdown: Shutdown , vms: &State<VirtualMachines>) -> ws::Channel<'_> {
    let mut vmids: Vec<Vmid> = Vec::new();

    ws.channel(move |mut channel| Box::pin(async move {
        let _ = channel.send("link standby...".into()).await;
        while let Some(message) = channel.next().await {
            let message = message?;
            if message.is_empty() | message.is_close() {
                break;
            }

            let next_port = if vmids.is_empty() {
                SocketAddr::from(([127, 0, 0, 1], 5900))
            } else {
                vmids[vmids.len() - 1].destination
            };

            match respond(message.into_text()?.as_str(), vms.config.pool.clone().unwrap_or_else(|| PathBuf::from("./pool")), vmids.len(), next_port) {
                Ok(Commands::Quit) => {
                    let _ = channel.send("Received 'EXIT': Server is Going down!".into()).await;
                    info!("Received 'EXIT': Server is Going down!");
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
                    error!("{}", err);
                    let _ = channel.send(err.into()).await;
                }
            }
        }
        if vmids.is_empty() {
            //skiping
        } else {
            info!("Saving {:#?}", vmids);
            let _ = new_pool(vms.config.pool.clone().unwrap_or_else(|| PathBuf::from("./pool")), vmids);
        }
        
        shutdown.clone().notify();
        Ok(())
    }))
}

fn respond(line: &str, pool_dir: PathBuf, next_id: usize, next_port: u16) -> Result<Commands, String> {
    info!("{}", line);
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    debug!("{:#?}", args);
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("quicksetup", matches)) => {
            let port = matches.get_one::<u16>("port").ok_or("Unknown".to_string())?;
            let vmids = matches.get_one::<usize>("vmids").ok_or("Unknown".to_string())?;

            new_pool(pool_dir, new::vmid(*port, *vmids)).map_err(|e| format!("{:?}", e))?;

            Ok(Commands::Message(format!("done! {}", vmids)))
        }
        Some(("new", matches)) | Some(("init", matches)) => {
            let port = **matches.get_one::<u16>("port").get_or_insert(&next_port);
            let vmid = **matches.get_one::<usize>("vmid").get_or_insert(&next_id);
            let name = matches.get_one::<String>("name").ok_or("`name`is required".to_string())?.to_string();
            let args = matches.get_many::<String>("args").ok_or("`args`is required".to_string())?.map(|vals| vals.to_owned()).collect::<Vec<_>>();

            Ok(Commands::Vmid(Vmid {
                vmid_number: vmid,
                name,
                port,
                qemu_arg: args,
                child: None,
                password: None,
                path: None,
            }))
        }
        Some(("ping", _matches)) => {
            Ok(Commands::Message("Pong".to_string()))
        }
        Some(("quit", _matches)) => {
            Ok(Commands::Quit)
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
                        .required(false)
                        .value_parser(value_parser!(usize)),
                    arg!(name: -n --name <NAME> "The name for your Virtual Machines.")
                        .required(true)
                        .value_parser(value_parser!(String)),
                    arg!(args: [ARGS] "the command line arguments for qemu.")
                        .num_args(1..)
                        .required(false)
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
