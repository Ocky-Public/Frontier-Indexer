use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveLocation {
    pub location_hash: Vec<u8>,
}

