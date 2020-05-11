use chrono::prelude::*;
use async_trait::async_trait;
use std::fmt;

use super::Module;
use crate::{config, GenResult};

#[derive(Default)]
pub struct Time {
    time_string: String,
}

impl Time {
    #[inline]
    fn get_datetime() -> String {
        let localtime: DateTime<Local> = Local::now();
        localtime.format("%A %v %I:%M%P").to_string()   // E.g: Sunday 02-01-2020 12:03pm
    }
}

#[async_trait]
impl Module for Time {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        let needs_update: bool = update_counter % config::TIME_UPDATE_PERIOD == 0;
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