use rocket::{tokio::{net::{TcpStream, UnixStream}, io::{AsyncReadExt, AsyncWriteExt}}, futures::{StreamExt, SinkExt}, State};
use std::{io::{self, ErrorKind, Error}, sync::{Mutex, Arc}, path::PathBuf, net::SocketAddr};
use crate::{execute::VirtualMachines, config::vmids::Vmid};
use serde::{Deserialize, Serialize};
use ws::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Destination {
    /// Connect to TCP
    Tcp(SocketAddr),

    /// Connect to unix domain socket
    #[cfg(unix)]
    Unix(PathBuf),
}

#[get("/stream/<streamfrom>")]
pub(crate) async fn stream(ws: ws::WebSocket, streamfrom: usize, vms: &State<VirtualMachines>) -> Result<ws::Channel<'static>, Error> {
    let mut buffer: Vec<u8> = vec![0; *vms.config.stream_buffer.clone().get_or_insert(10000)];
    let stream = getaddr(streamfrom, vms.virtual_machines.clone())?;

    //why is this the only way that i can get it to work.
    match stream {
        Destination::Tcp(stream) => {
            let mut stream = TcpStream::connect(stream).await?;
            Ok(ws.channel(move |mut channel| Box::pin(async move {loop {
                rocket::tokio::select! {
                    message = channel.next() => {
                        if let Some(message) = message {
                            let message = message?;
                            if message.is_binary() {
                                stream.write_all(&Message::binary(message).into_data()).await?;
                            }
                            else if message.is_close() {
                                channel.close(None).await?; 
                                return Ok(());
                            }
                        } else {
                            error!("No packet received from websocket");
                        }
                    },
                    data_bytes = stream.read(&mut buffer) => {
                        let data_bytes = data_bytes?;
                        if data_bytes > 0 {
                            channel.send(Message::binary(&buffer[0..data_bytes])).await?;
                        } 
                        else {
                            info!("TCP/Unix stream closed");
                            channel.close(None).await?;
                        }
                    }
                }
            }})))
        },
        Destination::Unix(stream) => {
            let mut stream = UnixStream::connect(stream).await?;
            Ok(ws.channel(move |mut channel| Box::pin(async move {loop {
                rocket::tokio::select! {
                    message = channel.next() => {
                        if let Some(message) = message {
                            let message = message?;
                            if message.is_binary() {
                                stream.write_all(&Message::binary(message).into_data()).await?;
                            }
                            else if message.is_close() {
                                channel.close(None).await?; 
                                return Ok(());
                            }
                        } else {
                            error!("No packet received from websocket");
                        }
                    },
                    data_bytes = stream.read(&mut buffer) => {
                        let data_bytes = data_bytes?;
                        if data_bytes > 0 {
                            channel.send(Message::binary(&buffer[0..data_bytes])).await?;
                        } 
                        else {
                            info!("TCP/Unix stream closed");
                            channel.close(None).await?;
                        }
                    }
                }
            }})))
        },
    }
}

fn getaddr(streamfrom: usize, virtual_machines: Vec<Arc<Mutex<Vmid>>>) -> io::Result<Destination> {
    if virtual_machines.len() > streamfrom {
        let vmid = &virtual_machines[streamfrom].lock().unwrap();

        Ok(vmid.destination.clone())
    } else {
        Err(Error::new(ErrorKind::NotFound,"The Requested VM Doesn't exist."))
    }
}

// #[test]
// fn test() -> Result<(), Error> {
//     use crate::config::vmids::new;
//     let vmid = new::vmid(5900)?.into_iter().map(|vals| Arc::new(Mutex::new(vals))).collect::<Vec<_>>();

//     //test 0
//     assert_eq!(getaddr(0, vmid.clone()).unwrap(), SocketAddr::from(([127, 0, 0, 1], 5900)));

//     //test 4
//     assert_eq!(getaddr(3, vmid.clone()).unwrap(), SocketAddr::from(([127, 0, 0, 1], 5903)));

//     assert_eq!(getaddr(10, vmid.clone()).unwrap_err().to_string(), "The Requested VM Doesn't exist.".to_string());
//     drop(vmid);
//     Ok(())
// }