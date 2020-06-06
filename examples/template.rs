/// A template to get started making new modules.
/// Don't forget to add:
/// - `pub mod template;` in `mod.rs`.
/// - `Box::new( modules::template::Template::default() )` to the array in `main.rs::main()`.


use super::*;
use crate::config;

pub struct Template {
    base_module: BaseModule,    // The BaseModule type provides basic methods that modules need.
    output: String,
}

impl Template {
    fn update_output(&mut self) {
        // Update stuff here
    }
}

impl Module for Template {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {   // If the module needs an update, update it.
            self.update_output();
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}

impl Default for Template {
    fn default() -> Self {
        Self {
            // Give the BaseModule the target update period for this module.
            base_module: BaseModule::new(config::TEMPLATE_UPDATE_PERIOD),
            output: String::new(),
        }
    }
}