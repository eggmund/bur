use super::{*, Module};
use crate::config;

use psutil::cpu::CpuPercentCollector;

pub struct CPUUsage {
    cpu_percent_collector: CpuPercentCollector,
    curr_cpu_percent: f32,
}

impl Default for CPUUsage {
    fn default() -> Self {
        Self {
            cpu_percent_collector: CpuPercentCollector::new().unwrap(),
            curr_cpu_percent: 0.0,
        }
    }
}

#[async_trait]
impl Module for CPUUsage {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        let needs_update = update_counter % config::CPU_USAGE_UPDATE_PERIOD == 0;
        if needs_update {
            self.curr_cpu_percent = self.cpu_percent_collector.cpu_percent()?;
            info!("New cpu percent: {}", self.curr_cpu_percent);
        }

        Ok(needs_update)
    }
}

impl fmt::Display for CPUUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}% CPU", self.curr_cpu_percent)
    }
}