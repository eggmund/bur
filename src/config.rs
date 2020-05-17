#![allow(dead_code)]

/// The minimum rate at which the bar updates. Default is 2.
pub const BASE_UPDATE_PERIOD: usize = 2;
/// The text used to separate modules. Default is "|".
pub const MODULE_SEPARATOR: &str = "|";

// UPDATE PERIODS: These values have to have a non-zero value
// For an update period of T seconds, you do T/BASE_UPDATE_PERIOD

// -- Time ---
pub const TIME_UPDATE_PERIOD: usize = 60/BASE_UPDATE_PERIOD;

// --- Binance ---
pub const BINANCE_UPDATE_PERIOD: usize = 60/BASE_UPDATE_PERIOD;
pub const BINANCE_SYMBOL_NUM: usize = 1;
pub const BINANCE_SYMBOLS: [&str; BINANCE_SYMBOL_NUM] = ["ETHUSDT"];
pub const BINANCE_TIMEOUT: u64 = 5;    // Seconds

// --- CPUUsage ---
pub const CPU_USAGE_UPDATE_PERIOD: usize = 2/BASE_UPDATE_PERIOD;

// --- MEM Usage ---
pub const MEM_USAGE_UPDATE_PERIOD: usize = 6/BASE_UPDATE_PERIOD;
