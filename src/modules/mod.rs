pub mod time;

pub use async_trait::async_trait;
pub use std::fmt;
pub use crate::GenResult;

/// Module is the trait for defining a new module.
/// The module has to implement the Display trait so that it can be displayed by the bar.
#[async_trait]
pub trait Module: fmt::Display {
    /// Returns `true` if the module has had to update, so that the bar knows to
    /// update the system's text.
    async fn update(&mut self, update_counter: usize) -> GenResult<bool>;
}
