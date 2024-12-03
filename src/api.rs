use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StockData {
    pub symbol: String,
    pub price: f64,
    pub percentage_change: f64,
}

pub async fn fetch_stock_data(symbol: &str) -> Result<StockData, String> {
    let api_key = std::env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY not set in .env");
    let url = format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/prev?adjusted=true&apiKey={}",
        symbol, api_key
    );

    let response = reqwest::get(&url).await.map_err(|e| format!("HTTP request failed: {}", e))?;
    let json: serde_json::Value = response.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Ensure we have results and parse the relevant fields
    if let Some(results) = json["results"].as_array().and_then(|r| r.first()) {
        let close_price = results["c"].as_f64().unwrap_or(0.0);
        let open_price = results["o"].as_f64().unwrap_or(0.0);
        let change_percent = ((close_price - open_price) / open_price) * 100.0;

        Ok(StockData {
            symbol: symbol.to_string(),
            price: close_price,
            percentage_change: change_percent,
        })
    } else {
        Err("API returned no results or invalid data.".to_string())
    }
}