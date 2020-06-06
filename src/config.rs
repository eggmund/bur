// The minimum rate at which the bar updates in milliseconds. Default is 1 second.
/// WARNING: This should be <= the smallest update period.
pub const BASE_UPDATE_PERIOD: u64 = 2000;
/// The text used to separate modules. Default is "|".
pub const MODULE_SEPARATOR: &str = "|";

// --- UPDATE PERIODS ---
// In milliseconds
#[cfg(feature = "time")]
pub const TIME_UPDATE_PERIOD: u64 = 1000 * 60;     // 1 second * 60

#[cfg(feature = "network")]
pub mod network {
    pub const NETWORK_UPDATE_PERIOD: u64 = 2000;
    pub const NETWORK_NO_SIGNAL_SYM: &str = "🚫";
    pub const NETWORK_CONFIGURING_SYM: &str = "-";
    pub const NETWORK_CONNECTED_SYM: &str = "📶";
}
#[cfg(feature = "network")]
pub use network::*;

