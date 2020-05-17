#[macro_use] extern crate log;

mod config;
mod modules;

use std::thread;
use std::time::Duration;

use modules::Module;

// Generic type for results that produce generic error.
pub type GenResult<T> = Result<T, Box<dyn std::error::Error>>;

struct Bur {
    modules: Vec<Box<dyn Module>>,
    update_counter: usize,
    base_update_sleep: Duration,
}

impl Bur {
    pub fn new(modules: Vec<Box<dyn Module>>) -> Self {
        Self {
            modules,
            update_counter: 0,
            base_update_sleep: Duration::new(config::BASE_UPDATE_PERIOD as u64, 0),
        }
    }

    pub async fn update(&mut self) -> GenResult<()> {
        let mut has_updated = false;    // true if any field has updated
        let mut bar_string = String::from(config::MODULE_SEPARATOR);
    
        for module in self.modules.iter_mut() {
            // Module can return error if it doesn't want to display anything
            match module.update(self.update_counter).await {
                Ok(module_needed_update) => {
                    bar_string.push_str(&format!(" {} {}", module, config::MODULE_SEPARATOR));

                    if module_needed_update {
                        has_updated = true;
                    }
                },
                Err(e) => debug!("Module update returned error: {}", e),
            }
        }

        if has_updated {
            self.update_bar_text(&bar_string);
            info!("Bar has updated: {}", &bar_string);
        }

        self.update_counter = self.update_counter.wrapping_add(1);
        thread::sleep(self.base_update_sleep);    // Sleep

        Ok(())
    }

    fn update_bar_text(&self, text: &str) {
        use std::process::Command;
        
        Command::new("xsetroot")
            .args(&["-name", text])
            .output()
            .expect("xsetroot command failed to start.");
    }
}


#[tokio::main]
async fn main() -> GenResult<()> {
    pretty_env_logger::init();
    // Have modules in order going from left -> right along bar
    // Place new modules inside Box
    let mut bur = Bur::new(vec![
        #[cfg(feature = "binance")]
        Box::new( modules::binance::Binance::default() ),
        // Box::new( modules::wifi::Wifi ),
        #[cfg(feature = "cpu_usage")]
        Box::new( modules::cpu_usage::CPUUsage::default() ),
        #[cfg(feature = "time")]
        Box::new( modules::time::Time::default() ),
    ]);

    loop {
        bur.update().await?;
    }
}
