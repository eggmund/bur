use super::*;
use crate::config;

use network_manager::{NetworkManager, ConnectionState};

pub struct Network {
    base_module: BaseModule,
    nm: NetworkManager,
    out_string: String,
}

impl Network {
    fn update_output(&mut self) -> ModuleResult<()> {
        let connections = self.nm.get_active_connections()?;
        self.out_string = if connections.len() == 0 {
            config::NETWORK_NO_SIGNAL_SYM
        } else {
            match connections[0].get_state()? {
                ConnectionState::Activated => config::NETWORK_CONNECTED_SYM,
                ConnectionState::Activating => config::NETWORK_CONFIGURING_SYM,
                _ => config::NETWORK_NO_SIGNAL_SYM,
            }
        }.to_owned();

        Ok(())
    }
}

impl Module for Network {
    fn update(&mut self, dt: &Duration) -> ModuleResult<bool> {
        let needs_update = self.base_module.needs_update(dt);
        if needs_update {
            info!("Updating network.");
            self.update_output()?;
        }
        Ok(needs_update)
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.out_string)
    }
}

impl Default for Network {
    fn default() -> Self {
        Self {
            nm: NetworkManager::new(),
            // Give the BaseModule the target update period for this module.
            base_module: BaseModule::new(config::NETWORK_UPDATE_PERIOD),
            out_string: String::new(),
        }
    }
}