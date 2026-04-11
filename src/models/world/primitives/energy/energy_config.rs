use diesel::prelude::*;
use serde::Deserialize;

use sui_indexer_alt_framework::FieldCount;
use sui_types::dynamic_field::Field;
use sui_types::object::Object;

use crate::schema::indexer::energy_config;

#[derive(Deserialize)]
pub struct MoveField {}

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = energy_config)]
pub struct StoredEnergyConfig {
    pub package_id: String,
    pub assembly_id: String,
    pub energy_cost: i64,
    pub entry_object_id: String,
    pub checkpoint_updated: i64,
}

impl StoredEnergyConfig {
    pub fn from_object(obj: &Object, owner_pkg_id: String, checkpoint_updated: i64) -> Self {
        let move_obj = obj.data.try_as_move().expect("Object is not a Move object");
        let bytes = move_obj.contents();

        let entry: Field<u64, u64> =
            bcs::from_bytes(bytes).expect("Failed to deserialze Entry object");

        Self {
            package_id: owner_pkg_id,
            assembly_id: entry.name.to_string(),
            energy_cost: entry.value as i64,
            entry_object_id: obj.id().to_string(),
            checkpoint_updated,
        }
    }
}
