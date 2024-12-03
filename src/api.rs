use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StockData {
    pub symbol: String,
    pub price: f64,
    pub percentage_change: f64,
}

pub async fn fetch_stock_data(symbol: &str) -> Result<StockData, Error> {
    let api_key = std::env::var("ALPHA_VANTAGE_KEY").expect("ALPHA_VANTAGE_KEY not set in .env");
    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        symbol, api_key
    );

    let response = reqwest::get(&url).await?;
    let json: serde_json::Value = response.json().await?;

    let quote = &json["Global Quote"];
    let stock_data = StockData {
        symbol: quote["01. symbol"].as_str().unwrap_or("").to_string(),
        price: quote["05. price"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
        percentage_change: quote["10. change percent"]
            .as_str()
            .unwrap_or("0%")
            .trim_end_matches('%')
            .parse()
            .unwrap_or(0.0),
    };

    Ok(stock_data)
}
