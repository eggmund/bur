use super::*;
use crate::config;

use std::io;

pub struct Bat {
    base_module: BaseModule,
    manager: battery::Manager,
    main_battery: battery::Battery,
    perc: f32,
    is_charging: bool,
}

impl Bat {
    pub fn new() -> battery::Result<Self> {
        let manager = battery::Manager::new()?;
        let battery = match manager.batteries()?.next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                warn!("Unable to access battery information.");
                return Err(e);
            }
            None => {
                warn!("Unable to find any batteries.");
                return Err(io::Error::from(io::ErrorKind::NotFound).into());
            }
        };

        Ok(Self {
            base_module: BaseModule::new(config::BAT_UPDATE_PERIOD),
            manager,
            main_battery: battery,
            perc: 0.0,
            is_charging: false,
        })
    }
}

impl Module for Bat {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            self.manager.refresh(&mut self.main_battery)?;
            self.perc = self.main_battery
                .state_of_charge()
                .into();
            self.perc *= 100.0;
            self.is_charging = self.main_battery.state() == battery::State::Charging;
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Bat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}% {}", self.perc, if self.is_charging { config::BAT_CHARGING_SYMBOL } else { config::BAT_SYMBOL })
    }
}