use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use sui_indexer_alt_framework::FieldCount;

use crate::schema::indexer::inventory_entries;

#[derive(Deserialize)]
pub struct MoveItemEntry {
    pub tenant: String,
    pub type_id: u64,
    pub item_id: u64,
    pub volume: u64,
    pub quantity: u32,
}

#[derive(Insertable, Serialize, Debug, Clone, FieldCount)]
#[diesel(table_name = inventory_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoredInventoryEntry {
    pub inventory_id: String,
    pub type_id: i64,
    pub item_id: String,
    pub volume: i64,
    pub quantity: i64,
    pub checkpoint_updated: i64,
}

impl StoredInventoryEntry {
    pub fn from_object(
        entry: &MoveItemEntry,
        inventory_id: String,
        checkpoint_updated: i64,
    ) -> Self {
        Self {
            inventory_id,
            type_id: entry.type_id as i64,
            item_id: entry.item_id.to_string(),
            volume: entry.volume as i64,
            quantity: entry.quantity as i64,
            checkpoint_updated,
        }
    }
}
