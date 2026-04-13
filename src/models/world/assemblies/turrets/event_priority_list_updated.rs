use serde::Deserialize;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Deserialize, Debug, Clone, Display, EnumString, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum MoveBehaviourChangeReason {
    Unspecified = 0,
    Entered = 1,
    StartedAttack = 2,
    StoppedAttack = 3,
}

#[derive(Deserialize)]
pub struct MoveTargetCandidate {
    item_id: u64,
    type_id: u64,
    group_id: u64,
    character_id: u32,
    character_tribe: u32,
    hp_ratio: u64,
    shield_ratio: u64,
    armor_ratio: u64,
    is_aggressor: bool,
    priority_weight: u64,
    behaviour_change: MoveBehaviourChangeReason,
}
