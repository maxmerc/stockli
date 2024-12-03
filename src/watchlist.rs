use std::collections::{HashSet, HashMap};
use crate::api::{fetch_stock_data, fetch_historical_data};
use crate::utils::calculate_ema;

#[derive(Default)]
pub struct Watchlist {
    stocks: HashSet<String>,                     // Set of stock symbols
    cached_data: HashMap<String, (f64, f64, f64, Vec<f64>)>, // Symbol -> (Open, Close, Change Percentage, EMA)
}

impl Watchlist {
    pub fn new() -> Self {
        Watchlist {
            stocks: HashSet::new(),
            cached_data: HashMap::new(),
        }
    }

    pub async fn add_stock(&mut self, symbol: String) -> Result<String, String> {
        if self.stocks.contains(&symbol) {
            return Err(format!("{} is already in your watchlist.", symbol));
        }

        let symbol_clone = symbol.clone();
        let historical_data = tokio::spawn(async move { fetch_historical_data(&symbol_clone).await })
            .await
            .map_err(|e| format!("Task join error: {}", e))?
            .map_err(|e| format!("Failed to fetch historical data: {}", e))?;

        let ema = calculate_ema(&historical_data, 14).ok_or("Not enough data for EMA calculation.")?;

        let symbol_clone = symbol.clone();
        let stock_data = tokio::spawn(async move { fetch_stock_data(&symbol_clone).await })
            .await
            .map_err(|e| format!("Task join error: {}", e))?
            .map_err(|e| format!("Failed to fetch stock data: {}", e))?;

        // Validate the fetched data
        if stock_data.open.is_nan() || stock_data.close.is_nan() || stock_data.percentage_change.is_nan()
            || stock_data.open == 0.0 || stock_data.close == 0.0
        {
            return Err(format!("Invalid data fetched for '{}'.", symbol));
        }

        self.stocks.insert(symbol.clone());
        self.cached_data.insert(
            stock_data.symbol.clone(),
            (stock_data.open, stock_data.close, stock_data.percentage_change, ema),
        );

        Ok(format!("Added {} to watchlist.", stock_data.symbol))
    }

    /// Remove a stock symbol from the watchlist.
    pub fn remove_stock(&mut self, symbol: &str) -> Result<String, String> {
        if self.stocks.remove(symbol) {
            self.cached_data.remove(symbol);
            Ok(format!("Removed {} from watchlist.", symbol))
        } else {
            Err(format!("{} is not in your watchlist.", symbol))
        }
    }

    pub async fn refresh_data(&mut self) -> Vec<String> {
        let mut messages = Vec::new();
        let mut tasks = Vec::new();

        for symbol in &self.stocks {
            let symbol = symbol.clone();
            let task = tokio::spawn(async move {
                let historical_data = fetch_historical_data(&symbol).await?;
                let ema = calculate_ema(&historical_data, 14).ok_or("Not enough data for EMA calculation.")?;
                let stock_data = fetch_stock_data(&symbol).await?;

                // Validate the fetched data
                if stock_data.open.is_nan() || stock_data.close.is_nan() || stock_data.percentage_change.is_nan()
                    || stock_data.open == 0.0 || stock_data.close == 0.0
                {
                    return Err(format!("Invalid data fetched for '{}'.", symbol));
                }

                Ok::<_, String>((symbol, stock_data, ema))
            });

            tasks.push(task);
        }

        for task in tasks {
            match task.await {
                Ok(Ok((symbol, stock_data, ema))) => {
                    self.cached_data.insert(
                        symbol.clone(),
                        (stock_data.open, stock_data.close, stock_data.percentage_change, ema),
                    );
                    messages.push(format!("Updated data for {}.", symbol));
                }
                Ok(Err(e)) => {
                    messages.push(format!("Failed to refresh data for a stock: {}", e));
                }
                Err(e) => {
                    messages.push(format!("Failed to join task: {}", e));
                }
            }
        }

        messages
    }

    pub fn get_cached_data(&self) -> Vec<(String, String, String, String, String)> {
        self.cached_data
            .iter()
            .map(|(symbol, &(open, close, change, ref ema))| (
                symbol.clone(),
                format!("${:.2}", open),
                format!("${:.2}", close),
                format!("{:.2}%", change),
                format!(
                    "[{}]",
                    ema.iter()
                        .map(|value| format!("{:.2}", value))
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            ))
            .collect()
    }
}

