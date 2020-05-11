use super::{*, Module};
use crate::config;

use binance_api::api::{Binance as BinanceAPI};
use binance_api::market::Market;

pub struct Binance {
    market: Market,
    current_prices: [f64; config::BINANCE_SYMBOL_NUM],
}

impl Default for Binance {
    fn default() -> Self {
        Self {
            market: BinanceAPI::new(None, None),
            current_prices: [0.0; config::BINANCE_SYMBOL_NUM],
        }
    }
}

#[async_trait]
impl Module for Binance {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        let needs_update = update_counter % config::BINANCE_UPDATE_PERIOD == 0;

        if needs_update {
            for (i, price) in self.current_prices.iter_mut().enumerate() {
                *price = self.market.get_price(config::BINANCE_SYMBOLS[i])?.price;
            }
        }

        Ok(needs_update)
    }
}

impl fmt::Display for Binance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_string = String::new();
        for (i, price) in self.current_prices.iter().enumerate() {
            out_string.push_str(&format!("{:.2}", price));

            match config::BINANCE_SYMBOLS[i] {
                "ETHUSDT" => out_string.push_str(" $ETH"),
                "BTCUSDT" => out_string.push_str(" $BTC"),
                x => out_string.push_str(x),
            }

            if i + 1 != config::BINANCE_SYMBOL_NUM { // Don't add separator if last one
                out_string.push_str(&format!(" {} ", config::MODULE_SEPARATOR));
            }
        }

        write!(f, "{}", out_string)
    }
}
