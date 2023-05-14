use bytes::Bytes;
use mini_redis::client::{self, Client};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();


    let task_1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let get_cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        tx.send(get_cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Received response from task_1::: {:#?}", res);
    });

    let task_2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let set_cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        tx2.send(set_cmd).await.unwrap();

        let res = resp_rx.await;
        println!("Received response from task_2 ::: {:#?}", res);
    });

    while let Some(msg) = rx.recv().await {
        println!("Message received ::: {:#?}", msg);
    }

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            by_command(cmd, &mut client).await;
        }
    });

    task_1.await.unwrap();
    task_2.await.unwrap();
    manager.await.unwrap();
}

async fn by_command(cmd: Command, client: &mut Client) {
    match cmd {
        Command::Get { key, resp } => {
            let res = client.get(&key).await;
            let _ = resp.send(res);
        }
        Command::Set { key, val, resp } => {
            let res = client.set(&key, val.clone()).await;
            let _ = resp.send(res);
        }
    }
}
