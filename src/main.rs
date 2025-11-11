use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct CoinPrice {
    usd: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,dogecoin,binancecoin,cardano,solana,polkadot,shiba-inu,litecoin&vs_currencies=usd";

    let response = reqwest::get(url).await?;
    let json: HashMap<String, CoinPrice> = response.json().await?;

    // 通貨名と価格のマッピングを表示
    for (coin, data) in json.iter() {
        println!("{}: ${:.2}", coin, data.usd);
    }

    println!("Fetched coin prices successfully.");

    Ok(())
}
