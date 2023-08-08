use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::structs::Message;
use anyhow::Result;
use futures_util::sink::SinkExt;
use futures_util::{stream::SplitSink, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub async fn network_processor(
    ui_to_network: &mut Receiver<Message>,
    network_to_ui: &mut Sender<Message>,
) {
    let mut connection_map: HashMap<
        u8,
        SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>,
    > = HashMap::new();

    let net_to_ui = network_to_ui;
    while let Some(message) = ui_to_network.recv().await {
        println!("Network = {:?}", message);
        match message {
            Message::NewClient { id, ip } => {
                let res = handle_new_client(net_to_ui.clone(), id.to_owned(), ip.to_owned()).await;
                match res {
                    Ok(sender) => {
                        connection_map.insert(id, sender);
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
            Message::Message {
                id,
                payload,
                num_bytes,
            } => {
                println!("id={}, payload={}, num_bytes={}", id, payload, num_bytes);
                match connection_map.get_mut(&id) {
                    Some(ws) => {
                        let _ = ws.send(tungstenite::Message::Text(payload)).await;
                    }
                    None => todo!(),
                }
            }
            Message::Close { id } => {
                if let Some(mut ws_sink) = connection_map.remove(&id) {
                    let _ = ws_sink.send(tungstenite::Message::Close(None)).await;
                    println!("Closed WebSocket for ID: {}", id);
                } else {
                    println!("Failed to find WebSocket for ID: {}", id);
                }
            }
        }
    }
}

pub async fn handle_new_client(
    network_to_ui: Sender<Message>,
    id: u8,
    ip: String,
) -> Result<
    SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>,
    anyhow::Error,
> {
    let net_to_ui = network_to_ui;
    let url = url::Url::parse(&ip)?;

    let (ws_stream, _) = connect_async(url).await?;

    let (write, mut read) = ws_stream.split();

    let idt = id.clone();
    tokio::spawn(async move {
        while let Some(result) = read.next().await {
            match result {
                Ok(message) => {
                    let data = message.into_data();
                    let num_bytes = data.len();
                    let message_string = String::from_utf8(data).unwrap();
                    println!("{}", &message_string);
                    let _ = net_to_ui
                        .send(Message::Message {
                            id: idt.to_owned(),
                            payload: message_string,
                            num_bytes,
                        })
                        .await;
                }
                Err(e) => {
                    eprintln!("Error reading message: {}", e);
                    break;
                }
            }
        }
    });
    Ok(write)
}
