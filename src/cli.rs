use crate::api;
use crate::watchlist::Watchlist;

pub async fn run() {
    let mut watchlist = Watchlist::new();

    loop {
        println!("Welcome to Stockli CLI!");
        println!("1. Add Stock");
        println!("2. Remove Stock");
        println!("3. View Watchlist");
        println!("4. Exit");

        let choice = get_input("Enter your choice: ");
        match choice.trim() {
            "1" => {
                let symbol = get_input("Enter stock symbol: ");
                watchlist.add_stock(symbol.trim().to_string());
            }
            "2" => {
                let symbol = get_input("Enter stock symbol to remove: ");
                watchlist.remove_stock(symbol.trim());
            }
            "3" => {
                for stock in watchlist.get_stocks() {
                    if let Ok(data) = api::fetch_stock_data(&stock).await {
                        println!(
                            "{}: ${} ({}%)",
                            data.symbol, data.price, data.percentage_change
                        );
                    }
                }
            }
            "4" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
