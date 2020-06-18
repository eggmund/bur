#![allow(dead_code)]

// The minimum rate at which the bar updates in milliseconds. Default is 1 second.
/// WARNING: This should be <= the smallest update period.
pub const BASE_UPDATE_PERIOD: u64 = 1000;
/// The text used to separate modules. Default is "|".
pub const MODULE_SEPARATOR: &str = "|";

// --- UPDATE PERIODS ---
// In milliseconds
pub const TIME_UPDATE_PERIOD: u64 = 1000 * 60;     // 1 second * 60

// Network
pub const NETWORK_UPDATE_PERIOD: u64 = 2000;
pub const NETWORK_NO_SIGNAL_SYM: &str = "🚫";
pub const NETWORK_CONFIGURING_SYM: &str = "-";
pub const NETWORK_CONNECTED_SYM: &str = "📶";

// Crypto prices. Supports Binance and Huobi exchanges.
pub const CRYPTO_UPDATE_PERIOD: u64 = 1000 * 60;
pub const CRYPTO_BINANCE_SYMBOLS: &[&str] = &["ETHUSDT", "BTCUSDT"];
pub const CRYPTO_HUOBI_SYMBOLS: &[&str] = &["RSRUSDT"];

// CPU usage
pub const CPU_UPDATE_PERIOD: u64 = 1000 * 2;

// Memory usage
pub const MEM_UPDATE_PERIOD: u64 = 1000 * 4;
