use serde::Deserialize;
use sui_sdk_types::Address;

#[derive(Deserialize)]
pub struct MoveMetadata {
    pub assembly_id: Address,
    pub name: String,
    pub description: String,
    pub url: String,
}
