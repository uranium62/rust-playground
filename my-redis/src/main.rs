use mini_redis::{client, Result};

/*
* Prerequisites:
*
* $ cargo install mini-redis
* $ mini-redis-server
*/


#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("[GET] {:?}", result.unwrap());

    Ok(())
}
