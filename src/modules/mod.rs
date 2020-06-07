#![allow(dead_code)]

#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "network")]
pub mod network;

#[cfg(feature = "binance_prices")]
pub mod binance_prices;

#[cfg(feature = "cpu")]
pub mod cpu;

#[cfg(feature = "mem")]
pub mod mem;

pub use std::fmt;
pub use std::time::Duration;

/// Module is the trait for defining a new module.
/// The module has to implement the Display trait so that it can be displayed by the bar.
pub trait Module: fmt::Display {
    /// Returns `true` if the module has had to update, so that the bar knows to
    /// update the system's text.
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool>;
}

/// Result type for use in module functions.
pub type ModuleResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Basic type needed for a module.
/// Handles when the module should update.
#[derive(Default)]
pub struct BaseModule {
    /// The current delta time counter.
    dt_counter: Duration,
    /// Target update period for this module.
    update_period: Duration,
    /// Has the first update been done?
    first_update_done: bool,
}

impl BaseModule {
    pub fn new(update_period_millis: u64) -> Self {
        Self {
            update_period: Duration::from_millis(update_period_millis),
            ..Default::default()
        }
    }

    pub fn needs_update(&mut self, dt: &Duration) -> bool {
        if !self.first_update_done {
            self.first_update_done = true;
            true
        } else if self.update_period > Duration::from_secs(0) {    // If the update period is 0 seconds, then this means the module never updates.
            self.dt_counter += *dt;
    
            if self.dt_counter > self.update_period {
                self.dt_counter -= self.update_period;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
