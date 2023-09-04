use crate::{config::vmids::{new::{self, DestinationOption}, Vmid}, pool::{new_pool, load_pool}, execute::VirtualMachines, websocket::stream::Destination};
use rocket::{futures::{StreamExt, SinkExt}, State};
use clap::{Command, arg, value_parser};
use std::{path::PathBuf, net::SocketAddr};
use rocket::Shutdown;

enum Commands {
    Quit,
    Load,
    SaveVmid(Vmid),
    Message(String),
    SaveVmidVec(Vec<Vmid>),
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

            match respond(message.into_text()?.as_str(), vmids.len()/*, channel*/) {
                Ok(Commands::Quit) => {
                    let _ = channel.send("Received 'EXIT': Server is Going down!".into()).await;
                    info!("Received 'EXIT': Server is Going down!");
                    channel.close(None).await?;
                }
                Ok(Commands::SaveVmid(vmid)) => {
                    vmids.push(vmid);
                    let _ = channel.send(format!("done {:#?}!", vmids).into()).await;
                    info!("done {:#?}!", vmids);
                }
                Ok(Commands::Message(message)) => {
                    let _ = channel.send(message.clone().into()).await;
                    info!("{}", message)
                }
                Ok(Commands::SaveVmidVec(mut vmidvec)) => {
                    vmids.append(&mut vmidvec)
                }
                Ok(Commands::Load) => {
                    vmids.append(&mut load_pool(vms.config.pool.clone().unwrap_or_else(|| PathBuf::from("./pool"))).unwrap());
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

fn respond(line: &str, next_id: usize/*, mut channel: DuplexStream*/) -> Result<Commands, String> {
    info!("{}", line);
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    debug!("{:#?}", args);
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("quicksetup", matches)) => {
            let port = matches.get_one::<u16>("port");
            let vmids = matches.get_one::<usize>("vmids").ok_or("Unknown".to_string())?;
            #[cfg(unix)]
            let unix = matches.get_one::<PathBuf>("unix");
            

            cfg_if::cfg_if! {
                if #[cfg(unix)] {
                    let destination_option = if port.is_some() {
                        DestinationOption::Tcp(*port.unwrap())
                    } else if unix.is_some() {
                        DestinationOption::Unix(*unix.unwrap())
                    } else {
                        panic!("THE FUCK??")
                    };
                } else {
                    let destination_option = DestinationOption::Tcp(*port.unwrap());
                }
            }

            Ok(Commands::SaveVmidVec(new::vmid(destination_option, *vmids).map_err(|e| format!("{:?}", e))?))
        }
        Some(("new", matches)) | Some(("init", matches)) => {
            let port = *matches.get_one::<u16>("port").unwrap();
            let vmid = *matches.get_one::<usize>("vmid").unwrap_or(&next_id);
            let name = matches.get_one::<String>("name").ok_or("`name`is required".to_string())?.to_string();
            let args = matches.get_many::<String>("args").ok_or("`args`is required".to_string())?.map(|vals| vals.to_owned()).collect::<Vec<_>>();

            Ok(Commands::SaveVmid(Vmid {
                vmid_number: vmid,
                name,
                destination: Destination::Tcp(SocketAddr::from(([127, 0, 0, 1], port))),
                qemu_arg: args,
                child: None,
                password: None,
                path: None,
            }))
        }
        Some(("load", _matches)) => {
            Ok(Commands::Load)
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
                    #[cfg(unix)]
                    arg!(unix: -u --unix <PORT> "the Directory for Unix Socket.")
                        .required(true)
                        .conflicts_with("port")
                        .value_parser(value_parser!(PathBuf)),
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
                    #[cfg(unix)]
                    arg!(unix: -u --unix <PORT> "the dir for Unix Socket.")
                        .required(false)
                        .conflicts_with("port")
                        .value_parser(value_parser!(PathBuf)),
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
            Command::new("ping")
                .about("Get a response")
                .hide(true)
                .help_template(APPLET_TEMPLATE),
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
            Command::new("load")
                .about("load the existing pool")
                .help_template(APPLET_TEMPLATE),
            ]
        )
}
