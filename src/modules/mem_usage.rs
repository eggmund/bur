use super::{*, Module};
use crate::config;

use psutil::cpu::CpuPercentCollector;

pub struct MemUsage {
    cpu_percent_collector: CpuPercentCollector,
    curr_mem_percent: f32,
}

impl Default for MemUsage {
    fn default() -> Self {
        Self {
            cpu_percent_collector: CpuPercentCollector::new().unwrap(),
            curr_cpu_percent: 0.0,
        }
    }
}

#[async_trait]
impl Module for MemUsage {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        let needs_update = update_counter % config::CPU_USAGE_UPDATE_PERIOD == 0;
        if needs_update {
            self.curr_mem_percent = self.cpu_percent_collector.cpu_percent()?;
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
