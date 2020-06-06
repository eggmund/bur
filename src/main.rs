#[macro_use] extern crate log;

mod config;
mod modules;

use std::thread;
use std::time::{Duration, Instant};

use modules::Module;

// Generic type for results that produce generic error.
pub type GenResult<T> = Result<T, Box<dyn std::error::Error>>;

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

    pub async fn update(&mut self) -> GenResult<()> {
        let mut has_updated = false;    // true if any field has updated
        let mut bar_string = String::from(config::MODULE_SEPARATOR);

        let update_time = Instant::now();
        let dt = update_time.duration_since(self.last_update_time);
        info!("dt: {:?}", dt);
    
        for module in self.modules.iter_mut() {
            // Module can return error if it doesn't want to display anything
            match module.update(&dt).await {
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

        self.last_update_time = update_time;

        let update_sleep = if dt > self.update_sleep { self.update_sleep - (dt - self.update_sleep) } else { self.update_sleep };
        info!("Update sleep: {:?}", update_sleep);
        thread::sleep(update_sleep);    // Sleep

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
        Box::new( modules::time::Time::default() ),
    ]);

    loop {
        bur.update().await?;
    }
}