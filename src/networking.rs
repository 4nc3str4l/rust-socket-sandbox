use tokio::sync::mpsc::{Receiver, Sender};

use crate::structs::Message;



pub async fn client_spawner(ui_to_network: &mut Receiver<Message>, network_to_ui: &mut Sender<Message>) {
    while let Some(message) = ui_to_network.recv().await  {
        println!("GOT = {:?}", message);
    }
}