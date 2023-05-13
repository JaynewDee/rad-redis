use mini_redis::client;
use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]

enum Command {
    Get {
        key: String
    },
    Set {
        key: String,
        val: Bytes
    }
    
}

#[tokio::main]
async fn main() {
   
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("Sent from sender1").await;
    });

    tokio::spawn(async move {
        tx2.send("Sent from sender2").await;
    });

    while let Some(msg) = rx.recv().await {
        println!("Message received ::: {}", msg);
    }
}
