pub mod time;

pub use async_trait::async_trait;
pub use std::fmt;
pub use std::time::Duration;

/// Module is the trait for defining a new module.
/// The module has to implement the Display trait so that it can be displayed by the bar.
#[async_trait]
pub trait Module: fmt::Display {
    /// Returns `true` if the module has had to update, so that the bar knows to
    /// update the system's text.
    async fn update(&mut self, dt: &Duration) -> ModuleResult<bool>;
}

/// Result type for use in module functions.
pub type ModuleResult<T> = Result<T, ModuleError>;

/// Error type containing message of what happened.
pub struct ModuleError(pub String);

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error msg: {}", self.0)
    }
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
        if self.update_period > Duration::from_secs(0) {    // If the update period is 0 seconds, then this means the module never updates.
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
