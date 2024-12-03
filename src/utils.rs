use yata::methods::EMA;
use yata::prelude::*;

/// Calculate the EMA for the given prices with a specified period
/// and return only the most recent 5 EMA values.
pub fn calculate_ema(prices: &[f64], period: usize) -> Option<Vec<f64>> {
    if prices.len() < period {
        return None;
    }

    let mut ema = EMA::new(period as u8, &prices[0]).unwrap();
    ema.next(&prices[0]);
    let mut ema_values = Vec::new();

    for &price in prices {
        ema_values.push(ema.next(&price));
    }

    // Only keep the most recent 5 EMA values
    let recent_ema = ema_values.into_iter().rev().take(5).collect::<Vec<_>>();
    Some(recent_ema.into_iter().rev().collect())
}
