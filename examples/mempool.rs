use mempoolspace::prelude::*;

#[tokio::main]
async fn main() {
    let url = Url::parse("https://mempool.space").unwrap();
    let client = MempoolClient::new(url);

    // Get prices
    let prices = client.get_prices().await.unwrap();
    println!("{:?}", prices);
}
