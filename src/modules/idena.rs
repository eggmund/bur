use super::{*, Module};
use crate::config;

use idena_api::IdenaAPI;

pub struct Idena {
    api: IdenaAPI,
    balance: f64,
}

impl Idena {
    fn get_api_key(path: &str) -> std::io::Result<String> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

impl Default for Idena {
    fn default() -> Self {
        let api = IdenaAPI::new(
            &Self::get_api_key(config::IDENA_API_KEY_FILE).unwrap(),
            config::IDENA_HOST_URL
        );

        Self {
            api,
            balance: 0.0,
        }
    }
}

#[async_trait]
impl Module for Idena {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        let needs_update = update_counter % config::IDENA_UPDATE_PERIOD == 0;

        if needs_update {
            let response = self.api.balance(config::IDENA_ADDRESS)
                .await?;

            self.balance = response["balance"].as_str().unwrap().parse::<f64>()? +
                response["stake"].as_str().unwrap().parse::<f64>()?;
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Idena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} DNA", self.balance)
    }
}