use std::str::FromStr;

use mempoolspace::prelude::*;

#[tokio::main]
async fn main() {
    let url = Url::parse("https://mempool.space").unwrap();
    let client = MempoolClient::new(url);

    // Get prices
    let prices = client.get_prices().await.unwrap();
    println!("{:?}", prices);

    // Get prices
    let address = Address::from_str("1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv").unwrap();
    let address = address.assume_checked();
    let stats = client.get_address(&address).await.unwrap();
    println!("{:?}", stats);
}
