use super::{*, Module};

pub struct Wifi;

#[async_trait]
impl Module for Wifi {
    async fn update(&mut self, update_counter: usize) -> GenResult<bool> {
        Ok(false)       
    }
}


impl fmt::Display for Wifi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "egg")
    }
}