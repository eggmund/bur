use super::*;
use crate::config;

pub struct Mem {
    base_module: BaseModule,
    perc: f32,
}

impl Module for Mem {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            self.perc = psutil::memory::virtual_memory()?
                .percent();
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Mem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:6.2} %MEM", self.perc)
    }
}

impl Default for Mem {
    fn default() -> Self {
        Self {
            base_module: BaseModule::new(config::MEM_UPDATE_PERIOD),
            perc: 0.0,
        }
    }
}