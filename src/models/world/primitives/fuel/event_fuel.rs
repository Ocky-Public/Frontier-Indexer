use serde::Deserialize;
use strum_macros::{AsRefStr, Display, EnumString};
use sui_sdk_types::Address;

use crate::models::world::MoveTenantItemId;

#[derive(Deserialize, Debug, Clone, Display, EnumString, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum MoveFuelAction {
    Deposited = 0,
    Withdrawn = 1,
    BurningStarted = 2,
    BurningStopped = 3,
    BurningUpdated = 4,
    Deleted = 5,
}

#[derive(Deserialize)]
pub struct MoveFuelEvent {
    pub assembly_id: Address,
    pub assembly_key: MoveTenantItemId,
    pub type_id: u64,
    pub old_quantity: u64,
    pub new_quantity: u64,
    pub is_burning: bool,
    pub action: MoveFuelAction,
}
