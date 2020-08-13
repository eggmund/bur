use super::*;
use crate::config;

use std::collections::BTreeMap;

// Uses coingecko
const URI: &str = "http://api.coingecko.com/api/v3";

pub struct Crypto {
    base_module: BaseModule,
    current_prices: BTreeMap<String, f64>,
    tokens: String,
}

impl Module for Crypto {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            let response = reqwest::blocking::get(&format!("{}/simple/price?ids={}&vs_currencies={}", URI, self.tokens, config::CRYPTO_VS_CURRENCY))?
                .json::<serde_json::Value>()?;

            let response_obj = response.as_object().unwrap();

            info!("Got prices: {:#?}", response_obj);

            for (token, price_data) in response_obj.iter() {
                self.current_prices.insert(
                    config::CRYPTO_TOKENS
                        .get::<str>(token)
                        .unwrap()
                        .to_string(),
                    price_data[config::CRYPTO_VS_CURRENCY].as_f64().unwrap(),
                );
            }
        }
        Ok(needs_update)
    }
}

impl Crypto {
    fn get_request_string() -> String {
        let mut out = String::new();

        for (id, _) in &config::CRYPTO_TOKENS {
            out += &format!("{}%2C", id);    // %2C = comma in web url
        }
        out.truncate(out.len() - 3);    // remove the last comma

        out
    }
}

impl fmt::Display for Crypto {
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
impl Default for Crypto {
    fn default() -> Self {
        Self {
            // Give the BaseModule the target update period for this module.
            base_module: BaseModule::new(config::CRYPTO_UPDATE_PERIOD),
            current_prices: BTreeMap::new(),
            tokens: Self::get_request_string(),
        }
    }
}
