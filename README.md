# Stock Watchlist CLI Application

A terminal-based stock watchlist application that allows users to add, remove, view, and refresh stock data with additional metrics like EMA (Exponential Moving Average).

---

## Features
- Add and remove stocks from your watchlist.
- View detailed stock information: open price, close price, percentage change, and EMA (last 5 days).
- Dynamically fetch and update stock data from Polygon.io's API.
- Clean and intuitive terminal-based interface using `ratatui`.

## Limitations
 - Please note that the API only allows 5 requests per minute. Therefore if you try and add a lot of stocks quickly it will not correctly work for thos after. Note each stock involves two requests as well, one for open and closing and percentage change and the other for historical data for EMA usage. Therefore must be careful utilizing more than two stocks at a time as some may not correctly update and will stay the same when refresh data is used. This can be solved by upgrading API plan but for the purposes of this project is out of scope.

---

## Prerequisites
1. **Rust**: Ensure Rust is installed on your system. You can install Rust using [rustup](https://rustup.rs/).
2. **API Key**: Obtain a free API key from [Polygon.io](https://polygon.io/). This is required to fetch stock data.

---

## Installation

### 1. Clone the Repository
```bash
git clone https://github.com/maxmerc/stockli.git
cd stockli
```
### 2. Set Up Environment Variables
Create a .env file in the root of the project directory and add your Polygon.io API key:

env

POLYGON_API_KEY = your_polygon_api_key_here

### 3. Install Dependencies
Ensure all dependencies are downloaded and updated:

~~~bash
cargo build
~~~
Running the Application
Run the application using:

~~~bash
cargo run
~~~
## Usage
- Navigate the Menu: Use the arrow keys (or mouse wheel) to move between menu options.
- Add a Stock: Enter a stock symbol (e.g., AAPL) and press Enter.
- Remove a Stock: Enter the stock symbol you wish to remove and press Enter.
- View Watchlist: See all stocks in your watchlist, including open/close prices, percentage change, and EMA.
- Refresh Data: Update all stock data in your watchlist.
- Exit: Exit the program.

## Dependencies
This project uses the following Rust crates:

1. ratatui: For terminal-based UI.
2. chrono: For date and time handling.
3. reqwest: For HTTP requests.
4. dotenv: For environment variable management.
5. serde: For JSON serialization/deserialization.
6. yata: For calculating EMA.
7. tokio: for async request handling

