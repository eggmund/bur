mod time;
pub use time::*;

use std::fmt::Display;
use async_trait::async_trait;

use crate::GenResult;

// Module is a trait for making the update systems modular, so it is easier to add modules.
// The module has to implement the Display trait so that it can be displayed by the bar.
// See std::fmt::Display in the rust docs for info.
#[async_trait]
pub trait Module: Display {
    // Returns true if it had to update.
    async fn update(&mut self, update_counter: usize) -> GenResult<bool>;
}
