use super::*;
use crate::config;

use chrono::prelude::*;

/// Included module with the default build.
/// Displays the current date and time in the format
/// `Weekday Day-Month-Year Time(am/pm)`, e.g: `Sunday 02-01-2020 12:03pm`.
pub struct Time {
    base_module: BaseModule,
    time_string: String,
}

impl Time {
    #[inline]
    fn update_datetime(&mut self) {
        let localtime: DateTime<Local> = Local::now();
        self.time_string = localtime.format("%A %v %I:%M%P").to_string();
    }
}

impl Module for Time {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            self.update_datetime();
            info!("New time: {}", self.time_string);
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time_string)
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            // Give the BaseModule the target update period for this module.
            base_module: BaseModule::new(config::TIME_UPDATE_PERIOD),
            time_string: String::new(),
        }
    }
}
