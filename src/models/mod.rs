use serde::{Deserialize, Serialize};
use std::str::FromStr;

use move_core_types::language_storage::StructTag;

pub mod app;
pub mod system;
pub mod world;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveTypeName {
    pub name: String,
}

impl MoveTypeName {
    pub fn to_components(&self) -> (String, String, String) {
        let type_name = if self.name.starts_with("0x") {
            self.name.clone()
        } else {
            format!("0x{}", &self.name)
        };

        let tag = StructTag::from_str(&type_name).expect("Could not parse TypeName into StructTag");

        let package_id = tag.address.to_canonical_string(true);
        let module_name = tag.module.to_string();
        let struct_name = tag.name.to_string();

        (package_id, module_name, struct_name)
    }
}
