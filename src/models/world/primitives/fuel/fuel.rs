use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveFuel {
    pub max_capacity: u64,
    pub burn_rate_in_ms: u64,
    pub type_id: Option<u64>,
    pub unit_volume: Option<u64>,
    pub quantity: u64,
    pub is_burning: bool,
    pub previous_cycle_elapsed_time: u64,
    pub burn_start_time: u64,
    pub last_updated: u64,
}
