use chrono::prelude::*;

use super::{*, Module};
use crate::config;

/// Included module with the default build.
/// Displays the current date and time in the format
/// `Weekday Day-Month-Year Time(am/pm)`, e.g: `Sunday 02-01-2020 12:03pm`.
pub struct Time {
    base_module: BaseModule,
    time_string: String,
}

impl Time {
    #[inline]
    fn get_datetime() -> String {
        let localtime: DateTime<Local> = Local::now();
        localtime.format("%A%v %I:%M%P").to_string()
    }
}

#[async_trait]
impl Module for Time {
    async fn update(&mut self, dt: &Duration) -> GenResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            self.time_string = Self::get_datetime();
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
            base_module: BaseModule::new(config::TIME_UPDATE_PERIOD),
            time_string: String::new(),
        }
    }
}