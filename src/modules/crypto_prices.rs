use super::*;
use crate::config;

use binance::market::Market;
use binance::api::*;
use std::collections::BTreeMap;
use std::path::Path;

use reqwest::Client;

pub struct CryptoPrices {
    base_module: BaseModule,
    market: Market,
    current_prices: BTreeMap<String, f64>,
}

impl Module for CryptoPrices {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            for symbol in config::BINANCE_SYMBOLS.iter() {
                self.current_prices.insert(
                    match *symbol {  // A few matches for common symbols
                        "ETHUSDT" => "Ξ".to_owned(),
                        "BTCUSDT" => "Ƀ".to_owned(),
                        "rsrusdt" => "#".to_owned(),
                        x => x.to_owned(),
                    },
                    match *symbol {
                        "rsrusdt" => huobi::get_price("rsrusdt")?,
                        x => self.market.get_price(x.to_owned())?.price,
                    },
                );
            }
        }
        Ok(needs_update)
    }
}

impl fmt::Display for CryptoPrices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_string = String::new();

        let n = self.current_prices.len();
        for (i, (symbol, price)) in self.current_prices.iter().enumerate() {
            out_string.push_str(&format!("{} {}", price, symbol));

            if i + 1 < n { // Add separator if it isn't the last one.
                out_string.push_str(&format!(" {} ", config::MODULE_SEPARATOR));
            }
        }

        write!(f, "{}", out_string)
    }
}
impl Default for CryptoPrices {
    fn default() -> Self {
        Self {
            // Give the BaseModule the target update period for this module.
            base_module: BaseModule::new(config::BINANCE_UPDATE_PERIOD),
            market: Binance::new(None, None),
            current_prices: BTreeMap::new(),
        }
    }
}


mod huobi {
    use crate::modules::ModuleResult;
    use std::fmt;

    pub fn get_price(symbol: &str) -> ModuleResult<f64> {
        if let Some(ticker) = reqwest::blocking::get(&format!("https://api.huobi.pro/market/detail/merged?symbol={}", symbol))?
                .json::<serde_json::Value>()?
                .get("tick")
        {
            Ok(ticker["close"].as_f64().unwrap())
        } else {
            Err(Box::new(HuobiError("Couldn't get ticker".to_owned())))
        }
    }

    #[derive(Debug)]
    struct HuobiError(String);

    impl std::error::Error for HuobiError {}

    impl fmt::Display for HuobiError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Huobi Error: {}", self.0)
        }
    }
}