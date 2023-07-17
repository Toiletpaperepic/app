use rocket::{tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}}, futures::{StreamExt, SinkExt}};
use ws::Message;
use std::{net::SocketAddr, io};

#[get("/stream")]
pub(crate) async fn stream(ws: ws::WebSocket) -> io::Result<ws::Channel<'static>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 5900));
    let mut stream = TcpStream::connect(addr).await?;
    let mut buffer: Vec<u8> = vec![0; 10000];

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