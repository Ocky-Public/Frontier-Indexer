use serde::{Deserialize, Serialize};

pub mod app;
pub mod system;
pub mod world;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveTypeName {
    pub name: String,
}
