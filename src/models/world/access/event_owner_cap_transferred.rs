use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;

use sui_indexer_alt_framework::FieldCount;
use sui_sdk_types::Address;
use sui_types::event::Event;

use crate::handlers::EventMeta;
use crate::schema::indexer::events_owner_cap_transferred;

#[derive(Deserialize)]
pub struct MoveOwnerCapTransferred {
    pub owner_cap_id: Address,
    pub authorized_object_id: Address,
    pub previous_owner: Address,
    pub owner: Address,
}

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = events_owner_cap_transferred)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoredOwnerCapTransferred {
    pub event_id: String,
    pub occurred_at: DateTime<Utc>,
    pub id: String,
    pub object_id: String,
    pub owner: String,
    pub owner_old: String,
}

impl StoredOwnerCapTransferred {
    pub fn from_event(event: &Event, meta: &EventMeta) -> Self {
        let move_event: MoveOwnerCapTransferred = bcs::from_bytes(&event.contents)
            .expect("Failed to deserialize OwnerCap Transferred event");

        let occurred_at = DateTime::from_timestamp_millis(meta.checkpoint_timestamp_ms())
            .expect("Failed to parse checkpoint timestamp into DateTime");

        Self {
            event_id: meta.event_digest(),
            occurred_at,
            id: move_event.owner_cap_id.to_hex(),
            object_id: move_event.authorized_object_id.to_hex(),
            owner: move_event.owner.to_hex(),
            owner_old: move_event.previous_owner.to_hex(),
        }
    }
}
