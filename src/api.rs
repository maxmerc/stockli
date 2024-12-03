use chrono::{Duration, Local, Datelike};
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StockData {
    pub symbol: String,
    pub open: f64,
    pub close: f64,
    pub percentage_change: f64,
}

#[derive(Deserialize, Debug)]
pub struct HistoricalDataPoint {
    pub close: f64,
}

pub async fn fetch_historical_data(symbol: &str) -> Result<Vec<f64>, String> {
    let api_key = std::env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY not set in .env");
    let current_date = (Local::now() - Duration::days(1)).format("%Y-%m-%d").to_string();
    let start_date = Local::now()
        .date_naive()
        .with_ordinal(1)
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let url = format!(
        "https://api.polygon.io/v2/aggs/ticker/{}/range/1/day/{}/{}?adjusted=true&sort=asc&apiKey={}",
        symbol, start_date, current_date, api_key,
    );

    let response = tokio::spawn(async move {
        reqwest::get(&url).await
    }).await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("HTTP request failed: {}", e))?;
    
    let json: serde_json::Value = tokio::spawn(async { response.json::<serde_json::Value>().await }).await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let results = json["results"]
        .as_array()
        .ok_or("No historical data available.")?
        .iter()
        .filter_map(|data_point| data_point["c"].as_f64()) // Extract the "c" (close price) field
        .collect::<Vec<f64>>();

    if results.is_empty() {
        Err("No valid close prices found in the historical data.".to_string())
    } else {
        Ok(results)
    }
}

pub async fn fetch_stock_data(symbol: &str) -> Result<StockData, String> {
    let api_key = std::env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY not set in .env");
    let current_date = (Local::now() - Duration::days(1)).format("%Y-%m-%d").to_string();
    let url = format!(
        "https://api.polygon.io/v1/open-close/{}/{}?adjusted=true&apiKey={}",
        symbol, current_date, api_key,
    );

    let response = tokio::spawn(async move {
        reqwest::get(url).await
    }).await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let json: serde_json::Value = tokio::spawn(async { response.json::<serde_json::Value>().await }).await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

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
