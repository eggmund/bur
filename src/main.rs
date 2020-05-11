#[macro_use] extern crate log;
extern crate binance as binance_api;

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
            let module_needed_update = module.update(self.update_counter).await?;
            bar_string.push_str(&format!(" {} {}", module, config::MODULE_SEPARATOR));

            if module_needed_update {
                has_updated = true;
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
        Box::new( modules::binance::Binance::default() ),
        Box::new( modules::idena::Idena::default() ),
        Box::new( modules::time::Time::default() ),
    ]);

    loop {
        bur.update().await?;
    }
}