use rocket::{tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}}, futures::{StreamExt, SinkExt}, State};
use std::{net::SocketAddr, io::{self, ErrorKind, Error}, sync::{Mutex, Arc}};
use crate::{execute::VirtualMachines, config::vmids::preload::Vmid};
use ws::Message;

#[get("/stream/<streamfrom>")]
pub(crate) async fn stream(ws: ws::WebSocket, streamfrom: usize, vms: &State<VirtualMachines>) -> io::Result<ws::Channel<'static>> {
    let mut buffer: Vec<u8> = vec![0; vms.config.stream_buffer];
    let addr = getaddr(streamfrom, vms.virtual_machines.clone())?;
    let mut stream = TcpStream::connect(addr).await?;

    Ok(ws.channel(move |mut channel| Box::pin(async move {loop {
        rocket::tokio::select! {
            message = channel.next() => {
                if let Some(message) = message {
                    let message = message?;
                    if message.is_binary() {
                        stream.write(&Message::binary(message).into_data()).await?;
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
                    let _ = channel.send(Message::binary(&buffer[0..data_bytes])).await?;
                } 
                else {
                    debug!("TCP/Unix stream closed");
                    channel.close(None).await?;
                }
            }
        }
    }})))
}

fn getaddr(streamfrom: usize, virtual_machines: Vec<Arc<Mutex<Vmid>>>) -> io::Result<SocketAddr>{
    if virtual_machines.len() > streamfrom {
        let vmid = &virtual_machines[streamfrom];
        let addr = SocketAddr::from(([127, 0, 0, 1], vmid.lock().unwrap().port));
        info!("addr = {}", addr);
            
        return Ok(addr);
    } else {
        return Err(Error::new(ErrorKind::NotFound,"The Requested VM Doesn't exist."));
    }
}

#[test]
fn test() {
    use crate::config::vmids::{preload, addmutex};
    let vmid = addmutex::run(preload::run(5900,4));

    //test 0
    assert_eq!(getaddr(0, vmid.clone()).unwrap(), SocketAddr::from(([127, 0, 0, 1], 5900)));

    //test 4
    assert_eq!(getaddr(3, vmid.clone()).unwrap(), SocketAddr::from(([127, 0, 0, 1], 5903)));

    assert_eq!(getaddr(10, vmid.clone()).unwrap_err().to_string(), "The Requested VM Doesn't exist.".to_string());
    drop(vmid)
}