/// The minimum rate at which the bar updates. Default is 2.
pub const BASE_UPDATE_PERIOD: usize = 2;
/// The text used to separate modules. Default is "|".
pub const MODULE_SEPARATOR: &str = "|";

// --- UPDATE PERIODS ---
// NOTE: These values have to have a non-zero value
// For an update period of T seconds, you do T/BASE_UPDATE_PERIOD
pub const TIME_UPDATE_PERIOD: usize = 60/BASE_UPDATE_PERIOD;
