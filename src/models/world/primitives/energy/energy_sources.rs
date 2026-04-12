use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveEnergySource {
    pub max_energy_production: u64,
    pub current_energy_production: u64,
    pub total_reserved_energy: u64,
}
