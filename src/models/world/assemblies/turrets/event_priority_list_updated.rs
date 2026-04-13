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
    pub item_id: u64,
    pub type_id: u64,
    pub group_id: u64,
    pub character_id: u32,
    pub character_tribe: u32,
    pub hp_ratio: u64,
    pub shield_ratio: u64,
    pub armor_ratio: u64,
    pub is_aggressor: bool,
    pub priority_weight: u64,
    pub behaviour_change: MoveBehaviourChangeReason,
}
