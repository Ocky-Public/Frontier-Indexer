use chrono::{DateTime, Utc};
use diesel::prelude::*;

use sui_indexer_alt_framework::FieldCount;

use crate::handlers::EventMeta;
use crate::models::world::MoveFuelEvent;
use crate::schema::indexer::events_fuel_deposited;

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = events_fuel_deposited)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoredFuelDeposited {
    pub event_id: String,
    pub occurred_at: DateTime<Utc>,
    pub id: String,
    pub item_id: String,
    pub tenant: String,
    pub type_id: i64,
    pub quantity_old: i64,
    pub quantity_new: i64,
    pub burning: bool,
}

impl StoredFuelDeposited {
    pub fn from_event(event: &MoveFuelEvent, meta: &EventMeta) -> Self {
        let occurred_at = DateTime::from_timestamp_millis(meta.checkpoint_timestamp_ms())
            .expect("Failed to parse checkpoint timestamp into DateTime");

        Self {
            event_id: meta.event_digest(),
            occurred_at,
            id: event.assembly_id.to_hex(),
            item_id: event.assembly_key.item_id.to_string(),
            tenant: event.assembly_key.tenant.to_string(),
            type_id: event.type_id as i64,
            quantity_old: event.old_quantity as i64,
            quantity_new: event.new_quantity as i64,
            burning: event.is_burning,
        }
    }
}
