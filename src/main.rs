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
        let mut bar_string = String::from("|");
    
        for module in self.modules.iter_mut() {
            let module_needed_update = module.update(self.update_counter).await?;
            bar_string.push_str(&format!(" {} |", module));

            if module_needed_update {
                has_updated = true;
            }
        }

        if has_updated {
            println!("Bar has updated.");
            self.update_bar_text(&bar_string);
        }

        self.update_counter += 1;
        thread::sleep(self.base_update_sleep);    // Sleep

        self.update_counter += 1;

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
    // Have modules in order going from left -> right along bar
    // Place new modules inside Box
    let mut bur = Bur::new(vec![
        Box::new( modules::Time::default() ),
    ]);

    loop {
        bur.update().await?;
    }
}