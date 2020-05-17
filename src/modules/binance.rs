use super::{*, Module};
use crate::config;

use std::collections::BTreeMap;
use tokio_binance::{MarketDataClient, BINANCE_US_URL};
use tokio::time::{Duration, timeout};

pub struct Binance {
    market: MarketDataClient,
    current_prices: BTreeMap<String, f64>,   // (symbol: price). Sorted by symbol
}

impl Default for Binance {
    fn default() -> Self {
        Self {
            market: MarketDataClient::connect("", BINANCE_US_URL).unwrap(),
            current_prices: BTreeMap::new(),
        }
    }
}

#[async_trait]
impl Module for Binance {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        use serde_json::Value;

        let needs_update = update_counter % config::BINANCE_UPDATE_PERIOD == 0;

        if needs_update {
            timeout(Duration::from_secs(config::BINANCE_TIMEOUT), self.market
                .get_price_ticker()
                .json::<Vec<Value>>()
            ).await??
                .into_iter()
                .filter(|val: &Value| config::BINANCE_SYMBOLS.contains(&val["symbol"].as_str().unwrap()))
                .for_each(|val| {
                    self.current_prices.insert(
                        val["symbol"].as_str().unwrap().to_owned(),
                        val["price"].as_str().unwrap().parse::<f64>().unwrap()
                    );
                });

            info!("Prices: {:?}", self.current_prices);
        }

        Ok(needs_update)
    }
}

impl fmt::Display for Binance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_string = String::new();

        let n = self.current_prices.len();
        for (i, (symbol, price)) in self.current_prices.iter().enumerate() {
            out_string.push_str(&format!("{:.2}", price));

            match symbol.as_ref() {
                "ETHUSDT" => out_string.push_str(" $ETH"),
                "BTCUSDT" => out_string.push_str(" $BTC"),
                x => out_string.push_str(x),
            }

            if i + 1 < n { // Add separator if it isn't the last one.
                out_string.push_str(&format!(" {} ", config::MODULE_SEPARATOR));
            }
        }

        write!(f, "{}", out_string)
    }
}
