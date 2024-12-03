mod cli;
mod api;
mod watchlist;
mod utils;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from a `.env` file if available.
    cli::run().await;
}