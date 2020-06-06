// A template to get started making new modules

use super::*;
use crate::config;

pub struct Template {
    base_module: BaseModule,
    out_string: String,
}

impl Template {
    fn update_output(&mut self) {
        self.out_string = "Out".into();
    }
}

impl Module for Template {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            self.update_output();
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.out_string)
    }
}

impl Default for Network {
    fn default() -> Self {
        Self {
            // Give the BaseModule the target update period for this module.
            base_module: BaseModule::new(config::TEMPLATE_UPDATE_PERIOD),
            out_string: String::new(),
        }
    }
}