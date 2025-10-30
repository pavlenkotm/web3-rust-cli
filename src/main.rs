use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Prices {
    solana: std::collections::HashMap<String, f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Fetching current SOL price from CoinGecko...");
    let resp = reqwest::blocking::get(
        "https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd"
    )?;
    let data: Prices = resp.json()?;
    let price = data.solana.get("usd").unwrap_or(&0.0);
    println!("💰 SOL price: ${:.2}", price);
    Ok(())
}
