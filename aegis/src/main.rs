mod system;
use crate::system::connection::connect;

#[tokio::main]
async fn main() {
    let _client = connect().await.expect("Failed to connect to database");

    println!("Connected from main");
}
