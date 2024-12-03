use std::collections::{HashSet, HashMap};
use crate::api::fetch_stock_data;

#[derive(Default)]
pub struct Watchlist {
    stocks: HashSet<String>,                     // Set of stock symbols
    cached_data: HashMap<String, (f64, f64, f64)>, // Symbol -> (Open, Close, Change Percentage)
}

impl Watchlist {
    pub fn new() -> Self {
        Watchlist {
            stocks: HashSet::new(),
            cached_data: HashMap::new(),
        }
    }

    /// Add a stock symbol to the watchlist and fetch its data.
    pub async fn add_stock(&mut self, symbol: String) -> Result<String, String> {
        if self.stocks.contains(&symbol) {
            return Err(format!("{} is already in your watchlist.", symbol));
        }
    
        match fetch_stock_data(&symbol).await {
            Ok(stock_data) => {
                self.stocks.insert(symbol.clone());
                self.cached_data.insert(
                    stock_data.symbol.clone(),
                    (stock_data.open, stock_data.close, stock_data.percentage_change),
                );
                Ok(format!("Added {} to watchlist.", stock_data.symbol))
            }
            Err(_) => Err(format!("Failed to fetch data for '{}'. Ensure it exists.", &symbol)),
        }
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

    /// Refresh all stocks in the watchlist.
    pub async fn refresh_data(&mut self) -> Vec<String> {
        let mut messages = Vec::new();

        for symbol in &self.stocks {
            match fetch_stock_data(symbol).await {
                Ok(stock_data) => {
                    self.cached_data.insert(
                        stock_data.symbol.clone(),
                        (stock_data.open, stock_data.close, stock_data.percentage_change),
                    );
                    messages.push(format!("Updated data for {}.", symbol));
                }
                Err(_) => {
                    messages.push(format!("Failed to fetch data for {}.", symbol));
                }
            }
        }

        messages
    }

    pub fn get_cached_data(&self) -> Vec<(String, String, String, String)> {
        self.cached_data
            .iter()
            .map(|(symbol, &(open, close, change))| (
                symbol.clone(),
                format!("${:.2}", open),
                format!("${:.2}", close),
                format!("{:.2}%", change),
            ))
            .collect()
    }
}
