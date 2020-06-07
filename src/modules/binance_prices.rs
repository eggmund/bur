use super::*;
use crate::config;

use binance::market::Market;
use binance::api::*;
use std::collections::BTreeMap;

pub struct BinancePrices {
    base_module: BaseModule,
    market: Market,
    current_prices: BTreeMap<String, f64>,
}

impl Module for BinancePrices {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            for symbol in config::BINANCE_SYMBOLS.iter() {
                self.current_prices.insert(
                    match *symbol {  // A few matches for common symbols
                        "ETHUSDT" => "Ξ".to_owned(),
                        "BTCUSDT" => "Ƀ".to_owned(),
                        x => x.to_owned(),
                    },
                    self.market.get_price(symbol.to_owned())?.price,
                );
            }
        }
        Ok(needs_update)
    }
}

impl fmt::Display for BinancePrices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_string = String::new();

        let n = self.current_prices.len();
        for (i, (symbol, price)) in self.current_prices.iter().enumerate() {
            out_string.push_str(&format!("{:.2} {}", price, symbol));

            if i + 1 < n { // Add separator if it isn't the last one.
                out_string.push_str(&format!(" {} ", config::MODULE_SEPARATOR));
            }
        }

        write!(f, "{}", out_string)
    }
}
impl Default for BinancePrices {
    fn default() -> Self {
        Self {
            // Give the BaseModule the target update period for this module.
            base_module: BaseModule::new(config::BINANCE_UPDATE_PERIOD),
            market: Binance::new(None, None),
            current_prices: BTreeMap::new(),
        }
    }
}