// The minimum rate at which the bar updates in milliseconds. Default is 60 seconds.
/// WARNING: This should be <= the smallest update period.
pub const BASE_UPDATE_PERIOD: u64 = 1000;// 1000 * 60;
/// The text used to separate modules. Default is "|".
pub const MODULE_SEPARATOR: &str = "|";

// --- UPDATE PERIODS ---
// In milliseconds
#[cfg(features = "time")]
pub const TIME_UPDATE_PERIOD: u64 = 1000; // 1000 * 60;     // 1 second * 60
