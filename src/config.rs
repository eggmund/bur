/// The minimum rate at which the bar updates. Default is 2.
pub const BASE_UPDATE_PERIOD: usize = 60;
/// The text used to separate modules. Default is "|".
pub const MODULE_SEPARATOR: &str = "|";

// --- UPDATE PERIODS ---
// NOTE: These values have to have a non-zero value
// For an update period of T seconds, you do T/BASE_UPDATE_PERIOD
pub const TIME_UPDATE_PERIOD: usize = 60/BASE_UPDATE_PERIOD;
pub const IDENA_UPDATE_PERIOD: usize = (60 * 30)/BASE_UPDATE_PERIOD;

// --- Idena ---
pub const IDENA_API_KEY_FILE: &str = "/home/josh/.config/Idena/node/datadir/api.key";
pub const IDENA_HOST_URL: &str = "http://localhost:9009/";
pub const IDENA_ADDRESS: &str = "0xd5da967d65bcafa164c996fdf99834c650be1e38";