use super::*;

#[derive(Default)]
pub struct HelloWorld;

#[async_trait]
impl Module for HelloWorld {
    async fn update(&mut self, _dt: &Duration) -> ModuleResult<bool> {
        // Since nothing needs to be updated since this module simply
        // prints "Hello, world!", just return false.
        Ok(false)
    }
}

// Implement the Display trait for this module so that it's output can be formatted.
impl fmt::Display for HelloWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hello, world!")
    }
}