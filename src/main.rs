use dotenv::dotenv;
use stockli::cli;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from a `.env` file if available.
    let _ = cli::run().await;
}
