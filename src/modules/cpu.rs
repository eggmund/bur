use super::*;
use crate::config;

use psutil::cpu::CpuPercentCollector;

pub struct Cpu {
    base_module: BaseModule,
    perc_collector: CpuPercentCollector,
    perc: f32,
}

impl Module for Cpu {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            self.perc = self.perc_collector.cpu_percent()?;
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:6.2} %CPU", self.perc)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            base_module: BaseModule::new(config::CPU_UPDATE_PERIOD),
            perc_collector: CpuPercentCollector::new().unwrap(),
            perc: 0.0,
        }
    }
}