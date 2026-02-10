use serde::{Deserialize, Serialize};

/// User configuration (persisted).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub das_delay_ms: u64,
    pub arr_delay_ms: u64,
    pub sd_arr_delay_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            das_delay_ms: 133,
            arr_delay_ms: 0,
            sd_arr_delay_ms: 0,
        }
    }
}
