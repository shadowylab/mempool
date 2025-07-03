use std::str::FromStr;

use mempoolspace::prelude::*;

#[tokio::main]
async fn main() {
    let url = Url::parse("https://mempool.space").unwrap();
    let client = MempoolClient::new(url);

    // Get block tip height
    let height = client.get_block_tip_height().await.unwrap();
    println!("Height: {height}");

    // Get mempool stats
    let stats = client.get_mempool().await.unwrap();
    println!("{stats:?}");

    // Get prices
    let prices = client.get_prices().await.unwrap();
    println!("{:?}", prices);

    // Get address stats
    let address = Address::from_str("1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv").unwrap();
    let address = address.assume_checked();
    let stats = client.get_address(&address).await.unwrap();
    println!("{:?}", stats);

    // Get recommended fees
    let fees = client.get_recommended_fees().await.unwrap();
    println!("{:?}", fees);
}
