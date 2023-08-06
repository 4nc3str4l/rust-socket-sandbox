use std::time::Duration;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::structs::{Message};



pub async fn network_processor(ui_to_network: &mut Receiver<Message>, network_to_ui: &mut Sender<Message>) {
    
    let net_to_ui = network_to_ui;
    while let Some(message) = ui_to_network.recv().await  {
        println!("Network = {:?}", message);
        let n = net_to_ui.clone();
        match message {
            Message::NewClient { id } => {
                tokio::spawn(async move {
                    let _ = n.send(Message::Message { id,  payload: "Hi".to_string() }).await;
                    tokio::time::sleep(Duration::from_secs(5)).await;
                });
            },
            Message::Message { id, payload } => {

            },
            Message::Close { id } => {

            },
        }

    }
}