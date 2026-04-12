use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;

use sui_indexer_alt_framework::FieldCount;
use sui_types::event::Event;

use crate::handlers::EventMeta;
use crate::schema::indexer::events_fuel_efficiency_removed;

#[derive(Deserialize)]
pub struct MoveFuelEfficiencyRemoved {
    fuel_type_id: u64,
}

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = events_fuel_efficiency_removed)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoredFuelEfficiencyRemoved {
    event_id: String,
    occurred_at: DateTime<Utc>,
    type_id: i64,
}

impl StoredFuelEfficiencyRemoved {
    pub fn from_event(event: &Event, meta: &EventMeta) -> Self {
        let move_event: MoveFuelEfficiencyRemoved = bcs::from_bytes(&event.contents)
            .expect("Failed to deserialize Fuel Efficiency Removed event");

        let occurred_at = DateTime::from_timestamp_millis(meta.checkpoint_timestamp_ms())
            .expect("Failed to parse checkpoint timestamp into DateTime");

        Self {
            event_id: meta.event_digest(),
            occurred_at,
            type_id: move_event.fuel_type_id as i64,
        }
    }
}
