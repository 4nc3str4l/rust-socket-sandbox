use tokio::sync::mpsc;



pub async fn packet_processor() {

    let (tx, mut rx) = mpsc::channel::<String>(32);

    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("Sending from first handle".to_owned()).await;
    });

    tokio::spawn(async move {
        tx2.send("Sending fromt the second handle".to_owned()).await;
    });

    while let Some(message) = rx.recv().await  {
        println!("GOT = {}", message);
    }
}