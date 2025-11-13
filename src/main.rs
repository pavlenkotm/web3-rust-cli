use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize, Debug)]
struct CoinGeckoPrices {
    solana: HashMap<String, f64>,
}

#[derive(Deserialize, Debug)]
struct CoinCapResponse {
    data: CoinCapAsset,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CoinCapAsset {
    price_usd: String,
}

#[derive(Deserialize, Debug)]
struct BinancePrice {
    price: String,
}

/// Fetch SOL price from CoinGecko API
fn fetch_from_coingecko(client: &reqwest::blocking::Client) -> Result<f64, Box<dyn std::error::Error>> {
    let mut url = "https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd".to_string();

    // Support API key if provided
    if let Ok(api_key) = env::var("COINGECKO_API_KEY") {
        url = format!("{}&x_cg_demo_api_key={}", url, api_key);
    }

    let resp = client.get(&url).send()?;
    let status = resp.status();
    let text = resp.text()?;

    if !status.is_success() {
        return Err(format!("CoinGecko API error: {}", status).into());
    }

    let data: CoinGeckoPrices = serde_json::from_str(&text)?;
    let price = *data.solana.get("usd").ok_or("Price not found")?;
    Ok(price)
}

/// Fetch SOL price from CoinCap API
fn fetch_from_coincap(client: &reqwest::blocking::Client) -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.coincap.io/v2/assets/solana";
    let resp = client.get(url).send()?;
    let status = resp.status();
    let text = resp.text()?;

    if !status.is_success() {
        return Err(format!("CoinCap API error: {}", status).into());
    }

    let data: CoinCapResponse = serde_json::from_str(&text)?;
    let price = data.data.price_usd.parse::<f64>()?;
    Ok(price)
}

/// Fetch SOL price from Binance API
fn fetch_from_binance(client: &reqwest::blocking::Client) -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT";
    let resp = client.get(url).send()?;
    let status = resp.status();
    let text = resp.text()?;

    if !status.is_success() {
        return Err(format!("Binance API error: {}", status).into());
    }

    let data: BinancePrice = serde_json::from_str(&text)?;
    let price = data.price.parse::<f64>()?;
    Ok(price)
}

/// Try fetching price from multiple sources with fallback
fn fetch_sol_price() -> Result<f64, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Try CoinGecko first
    println!("🔗 Trying CoinGecko API...");
    match fetch_from_coingecko(&client) {
        Ok(price) => {
            println!("✅ CoinGecko: Success");
            return Ok(price);
        }
        Err(e) => {
            eprintln!("⚠️  CoinGecko failed: {}", e);
        }
    }

    // Fallback to CoinCap
    println!("🔗 Trying CoinCap API...");
    match fetch_from_coincap(&client) {
        Ok(price) => {
            println!("✅ CoinCap: Success");
            return Ok(price);
        }
        Err(e) => {
            eprintln!("⚠️  CoinCap failed: {}", e);
        }
    }

    // Fallback to Binance
    println!("🔗 Trying Binance API...");
    match fetch_from_binance(&client) {
        Ok(price) => {
            println!("✅ Binance: Success");
            return Ok(price);
        }
        Err(e) => {
            eprintln!("⚠️  Binance failed: {}", e);
        }
    }

    Err("❌ All API sources failed. Please check your internet connection or try again later.".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Web3 Rust CLI - Solana Price Checker");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    match fetch_sol_price() {
        Ok(price) => {
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("💰 SOL price: ${:.2}", price);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            Ok(())
        }
        Err(e) => {
            eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            eprintln!("{}", e);
            eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            eprintln!("\n💡 Tip: Set COINGECKO_API_KEY environment variable if you have one");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_display_format() {
        let price = 123.456789;
        let formatted = format!("${:.2}", price);
        assert_eq!(formatted, "$123.46");
    }

    #[test]
    fn test_environment_variable_handling() {
        // Test that env var access doesn't panic
        let _ = env::var("COINGECKO_API_KEY");
    }
}
