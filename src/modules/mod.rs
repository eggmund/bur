pub mod time;

pub use async_trait::async_trait;
pub use std::fmt;
pub use std::time::Duration;
pub use crate::GenResult;

/// Module is the trait for defining a new module.
/// The module has to implement the Display trait so that it can be displayed by the bar.
#[async_trait]
pub trait Module: fmt::Display {
    /// Returns `true` if the module has had to update, so that the bar knows to
    /// update the system's text.
    async fn update(&mut self, dt: &Duration) -> GenResult<bool>;
}

/// Basic type needed for a module.
/// Handles when the module should update.
#[derive(Default)]
pub struct BaseModule {
    dt_counter: Duration,
    update_period: Duration,
}

impl BaseModule {
    pub fn new(update_period_millis: u64) -> Self {
        Self {
            update_period: Duration::from_millis(update_period_millis),
            ..Default::default()
        }
    }

    pub fn needs_update(&mut self, dt: &Duration) -> bool {
        self.dt_counter += *dt;
    
        if self.dt_counter > self.update_period {
            self.dt_counter -= self.update_period;
            true
        } else {
            false
        }
    }
}
