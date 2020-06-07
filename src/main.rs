#[macro_use] extern crate log;

mod config;
mod modules;

use std::thread;
use std::time::{Duration, Instant};

use modules::Module;


struct Bur {
    modules: Vec<Box<dyn Module>>,
    last_update_time: Instant,
    update_sleep: Duration,
}

impl Bur {
    pub fn new(modules: Vec<Box<dyn Module>>) -> Self {
        Self {
            modules,
            last_update_time: Instant::now(),
            update_sleep: Duration::from_millis(config::BASE_UPDATE_PERIOD),
        }
    }

    pub fn update(&mut self) {
        let mut has_updated = false;    // true if any field has updated
        let mut bar_string = String::from(config::MODULE_SEPARATOR);

        let update_time = Instant::now();
        let dt = update_time.duration_since(self.last_update_time);
        debug!("dt: {:?}", dt);
    
        for module in self.modules.iter_mut() {
            // Module can return error if it doesn't want to display anything
            match module.update(&dt) {
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
            debug!("Bar has updated: {}", &bar_string);
        }

        self.last_update_time = update_time;

        thread::yield_now();
        thread::sleep(self.update_sleep);
    }

    fn update_bar_text(&self, text: &str) {
        use std::process::Command;
        
        Command::new("xsetroot")
            .args(&["-name", text])
            .output()
            .expect("xsetroot command failed to start. Please make sure it is installed.");
    }
}


fn main() {
    #[cfg(feature = "logger")]
    pretty_env_logger::init();

    // Have modules in order going from left -> right along bar
    // Place new modules inside Box
    let mut bur = Bur::new(vec![
        #[cfg(feature = "binance_prices")]
        Box::new( modules::binance_prices::BinancePrices::default() ),

        #[cfg(feature = "cpu")]
        Box::new( modules::cpu::Cpu::default() ),

        #[cfg(feature = "mem")]
        Box::new( modules::mem::Mem::default() ),

        #[cfg(feature = "network")]
        Box::new( modules::network::Network::default() ),

        #[cfg(feature = "time")]
        Box::new( modules::time::Time::default() ),
    ]);

    loop {
        bur.update();
    }
}