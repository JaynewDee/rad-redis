use std::collections::HashMap;

use mini_redis::{Connection, Frame};
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();
       
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = from_command(frame, &db);
       
        connection.write_frame(&response).await.unwrap();
    }
}

// Tasks in Tokio are very lightweight.
// Under the hood, they require only a single allocation and 64 bytes of memory.
// Applications should feel free to spawn thousands, if not millions of tasks.

// NOTE on lifetimes:
// When you spawn a task on the Tokio runtime, its type's lifetime must be 'static.
// This means that the spawned task must not contain any references to data owned outside the task.

//
//
//

fn from_command(frame: Frame, db: &Db) -> Frame {
    use mini_redis::Command::{self, Get, Set};

    match Command::from_frame(frame).unwrap() {
        Set(cmd) => {
            let mut db = db.lock().unwrap();
            // The value is stored as `Vec<u8>`
            db.insert(cmd.key().to_string(), cmd.value().clone());
            Frame::Simple("OK".to_string())
        }
        Get(cmd) => {
            let db = db.lock().unwrap();
            if let Some(value) = db.get(cmd.key()) {
                // `Frame::Bulk` expects data to be of type `Bytes`. This
                // type will be covered later in the tutorial. For now,
                // `&Vec<u8>` is converted to `Bytes` using `into()`.
                Frame::Bulk(value.clone().into())
            } else {
                Frame::Null
            }
        }
        cmd => panic!("unimplemented {:?}", cmd),
    }
}
