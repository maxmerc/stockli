use std::collections::HashSet;

#[derive(Default)]
pub struct Watchlist {
    stocks: HashSet<String>,
}

impl Watchlist {
    pub fn new() -> Self {
        Watchlist {
            stocks: HashSet::new(),
        }
    }

    pub fn add_stock(&mut self, symbol: String) {
        if self.stocks.insert(symbol.clone()) {
            println!("Added {} to watchlist.", symbol);
        } else {
            println!("{} is already in your watchlist.", symbol);
        }
    }

    pub fn remove_stock(&mut self, symbol: &str) {
        if self.stocks.remove(symbol) {
            println!("Removed {} from watchlist.", symbol);
        } else {
            println!("{} is not in your watchlist.", symbol);
        }
    }

    pub fn get_stocks(&self) -> Vec<String> {
        self.stocks.iter().cloned().collect()
    }
}
