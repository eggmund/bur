use super::*;
use crate::config;

use std::collections::BTreeMap;

// Uses coingecko
const URI: &str = "http://api.coingecko.com/api/v3";

pub struct CryptoPrices {
    base_module: BaseModule,
    current_prices: BTreeMap<String, f64>,
    tokens: String,
}

impl Module for CryptoPrices {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            let response = reqwest::blocking::get(&format!("{}/simple/price?ids={}&vs_currencies={}", URI, self.tokens, config::CRYPTO_VS_CURRENCY))?
                .json::<serde_json::Value>()?;

            let response_obj = response.as_object().unwrap();

            info!("Got prices: {:#?}", response_obj);

            for (token, price_data) in response_obj.into_iter() {
                self.current_prices.insert(
                    match token.as_ref() {  // A few matches for common symbols (and a few I track, feel free to add your own)
                        "ethereum" => "Ξ".to_owned(),
                        "bitcoin" => "Ƀ".to_owned(),
                        "reserve-rights-token" => "#".to_owned(),
                        "akropolis" => "₳".to_owned(),
                        x => {
                            let mut owned = x.to_owned();
                            owned.truncate(4);
                            owned
                        },
                    },
                    price_data[config::CRYPTO_VS_CURRENCY].as_f64().unwrap(),
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
            base_module: BaseModule::new(config::CRYPTO_UPDATE_PERIOD),
            current_prices: BTreeMap::new(),
            tokens: config::CRYPTO_TOKENS.replace(",", "%2C"),
        }
    }
}
