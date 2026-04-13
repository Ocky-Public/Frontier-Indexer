use serde::Deserialize;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Deserialize, Debug, Clone, Copy, Display, EnumString, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum MoveStatus {
    Null = 0,
    Offline = 1,
    Online = 2,
}

#[derive(Deserialize, Debug, Clone, Copy, Display, EnumString, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum MoveAction {
    Anchored = 0,
    Online = 1,
    Offline = 2,
    Unanchored = 3,
}

#[derive(Deserialize)]
pub struct MoveAssemblyStatus {
    pub status: MoveStatus,
}
