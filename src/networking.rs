use std::time::Duration;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::structs::Message;



pub async fn network_processor(ui_to_network: &mut Receiver<Message>, network_to_ui: &mut Sender<Message>) {
    
    let net_to_ui = network_to_ui;
    while let Some(message) = ui_to_network.recv().await  {
        println!("Network = {:?}", message);
        match message {
            Message::NewClient { id, ip } => {
                handle_new_client(net_to_ui.clone(), id.to_owned(), ip.to_owned());
            },
            Message::Message { id, payload } => {
                // Here I need somehow to have websocket handles to choose one and give him the data 
            },
            Message::Close { id } => {
                // Here I need to close the socket and remove it from the list
            },
        }

    }
}


pub fn handle_new_client(network_to_ui: Sender<Message>, id: String, ip: String) {
    let net_to_ui = network_to_ui;
    tokio::spawn(async move {
        let _ = net_to_ui.send(Message::Message { id,  payload: "Hi".to_string() }).await;
        tokio::time::sleep(Duration::from_secs(5)).await;
    });
}