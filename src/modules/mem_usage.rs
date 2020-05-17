use super::{*, Module};
use crate::config;

use psutil::memory::{self, VirtualMemory};

pub struct MemUsage {
    virt_mem: VirtualMemory,
    curr_mem_percent: f32,
}

impl Default for MemUsage {
    fn default() -> Self {
        Self {
            virt_mem: memory::virtual_memory().unwrap(),
            curr_mem_percent: 0.0,
        }
    }
}

#[async_trait]
impl Module for MemUsage {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        let needs_update = update_counter % config::MEM_USAGE_UPDATE_PERIOD == 0;
        if needs_update {
            self.curr_mem_percent = self.virt_mem.percent();
            info!("New MEM percent: {}", self.curr_mem_percent);
        }

        Ok(needs_update)
    }
}

impl fmt::Display for MemUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}% MEM", self.curr_mem_percent)
    }
}
