use mini_redis::{client, Result};

// This macro transforms the function at compilation, initializing the runtime and 
// resulting in a code block that looks something like this:
// fn main() {
//     let mut runtime = tokio::runtime::Runtime::new().unwrap();
//     runtime.block_on(async {
//         ... function body ...
//     })
// }
#[tokio::main]
async fn main() -> Result<()> {
    // Under the hood, `connect` takes a generic argument T where T implements the `ToSocketAddrs`
    //  trait
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Arg1 -> key | Arg2 -> value
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("Value retrieved from redis server\n:::{:#?}", &result);

    // Without the .await operator, an async function call simply
    // returns a value representing the operation.
    // The return value of an async fn is an anonymous type that implements the `Future` trait!
    let async_op = say_async("Hello, squirrel!");

    // Here the fn is actually executed
    async_op.await;

    Ok(())
}

async fn say_async(text: &str) {
    println!("{}", text);
}
