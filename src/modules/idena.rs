use super::{*, Module};
use crate::config;
use std::collections::VecDeque;

use idena_api::IdenaAPI;

pub struct Idena {
    api: IdenaAPI,
    balance: f64,
    previous_balances: VecDeque<f64>,
    dna_rate: f64,
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

impl Idena {
    pub fn update_dna_rate(&mut self) {
        if self.previous_balances.len() > 1 {
            let mut diffs = Vec::new();
            for i in 0..self.previous_balances.len()-1 {
                diffs.push(self.previous_balances[i] - self.previous_balances[i + 1]);
            }
    
            // Get mean value for every update. dna over update. dna/update time in secs = dna per sec
            let n = diffs.len() as f64;
            let mean_diff = diffs.into_iter()
                .fold(0.0, |sum, x| sum + x)/n;

            info!("Mean diff: {}", mean_diff);
    
            self.dna_rate = (mean_diff/(config::BASE_UPDATE_PERIOD * config::IDENA_UPDATE_PERIOD) as f64) * 60.0 * 60.0 * 24.0;
        }
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
            previous_balances: VecDeque::new(),
            dna_rate: 0.0,
        }
    }
}

#[async_trait]
impl Module for Idena {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        let needs_update = update_counter % config::IDENA_UPDATE_PERIOD == 0;

        if needs_update {
            let balance_response = self.api.balance(config::IDENA_ADDRESS)
                .await?;

            if self.balance > 0.0 {
                self.previous_balances.push_front(self.balance);
                self.previous_balances.truncate(config::IDENA_PREV_BALANCES);
            }

            info!("Response from idena node: {:#?}", balance_response);
            self.balance = balance_response["balance"].as_str().unwrap().parse::<f64>()? +
                balance_response["stake"].as_str().unwrap().parse::<f64>()?;

            self.update_dna_rate();
            info!("dna/day: {}", self.dna_rate);
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Idena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} DNA -- {:.3} DNA/day", self.balance, self.dna_rate)
    }
}