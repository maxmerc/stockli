use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StockData {
    pub symbol: String,
    pub open: f64,
    pub close: f64,
    pub percentage_change: f64,
}

pub async fn fetch_stock_data(symbol: &str) -> Result<StockData, String> {
    let api_key = std::env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY not set in .env");
    let url = format!(
        "https://api.polygon.io/v1/open-close/{symbol}/2023-12-01?adjusted=true&apiKey={}",
        api_key
    );

    let response = reqwest::get(&url).await.map_err(|e| format!("HTTP request failed: {}", e))?;
    let json: serde_json::Value = response.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let open = json["open"].as_f64().unwrap_or(0.0);
    let close = json["close"].as_f64().unwrap_or(0.0);
    let percentage_change = ((close - open) / open) * 100.0;

    Ok(StockData {
        symbol: symbol.to_string(),
        open,
        close,
        percentage_change,
    })
}