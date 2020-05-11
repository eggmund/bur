// Bar config
pub const BASE_UPDATE_PERIOD: usize = 2;
// NOTE: These values have to have a non-zero value
// For an update period of T seconds, you do T/BASE_UPDATE_PERIOD
pub const TIME_UPDATE_PERIOD: usize = 60/BASE_UPDATE_PERIOD;
