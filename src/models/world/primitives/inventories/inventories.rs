use serde::Deserialize;

use diesel::prelude::*;

use sui_indexer_alt_framework::FieldCount;
use sui_types::collection_types::VecMap;
use sui_types::object::Object;
use sui_types::object::Owner;

use crate::models::world::MoveItemEntry;
use crate::schema::indexer::inventories;

#[derive(Deserialize)]
pub struct MoveInventory {
    pub max_capacity: u64,
    pub used_capacity: u64,
    pub items: VecMap<u64, MoveItemEntry>,
}

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = inventories)]
pub struct StoredInventory {
    pub id: String,
    pub parent_id: String,
    pub capacity_used: i64,
    pub capacity_max: i64,
    pub checkpoint_updated: i64,
}

impl StoredInventory {
    pub fn from_object(&self, obj: Object, checkpoint_updated: i64) -> Result<Self, String> {
        let move_obj = obj.data.try_as_move().expect("Object is not a Move object");
        let bytes = move_obj.contents();

        let inventory: MoveInventory =
            bcs::from_bytes(bytes).expect("Failed to deserialize Inventory object");

        let Owner::ObjectOwner(owner) = obj.owner else {
            return Err(String::from("Expected object owner for inventory."));
        };

        let parent_id = owner.to_string();

        Ok(Self {
            id: obj.id().to_canonical_string(true),
            parent_id,
            capacity_max: inventory.max_capacity as i64,
            capacity_used: inventory.used_capacity as i64,
            checkpoint_updated,
        })
    }
}
