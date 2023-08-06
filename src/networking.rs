use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::structs::Message;
use anyhow::Result;
use futures_util::stream::SplitSink;
use futures_util::{future, pin_mut, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub async fn network_processor(
    ui_to_network: &mut Receiver<Message>,
    network_to_ui: &mut Sender<Message>,
) {
    let mut connection_map: HashMap<
        String,
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
                        connection_map.insert(id.to_owned(), sender);
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
            Message::Message { id, payload } => {
                // Here I need somehow to have websocket handles to choose one and give him the data
            }
            Message::Close { id } => {
                // Here I need to close the socket and remove it from the list
            }
        }
    }
}

pub async fn handle_new_client(
    network_to_ui: Sender<Message>,
    id: String,
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
                    let message_string = String::from_utf8(data).unwrap();
                    println!("{}", &message_string);
                    let _ = net_to_ui
                        .send(Message::Message {
                            id: idt.to_owned(),
                            payload: message_string,
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
