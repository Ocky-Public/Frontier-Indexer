use chrono::DateTime;
use diesel::prelude::*;
use serde::Deserialize;

use sui_indexer_alt_framework::FieldCount;
use sui_sdk_types::Address;
use sui_types::event::Event;

use crate::handlers::EventMeta;
use crate::schema::indexer::events_owner_cap_transferred;

#[derive(Deserialize)]
pub struct MoveOwnerCapTransferred {
    owner_cap_id: Address,
    authorized_object_id: Address,
    previous_owner: Address,
    owner: Address,
}

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = events_owner_cap_transferred)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoredOwnerCapTransferred {
    pub occurred_at: DateTime<chrono::Utc>,
    pub id: String,
    pub object_id: String,
    pub previous_owner: String,
    pub owner: String,
}

impl StoredOwnerCapTransferred {
    pub fn from_event(event: &Event, meta: &EventMeta) -> Self {
        let move_event: MoveOwnerCapTransferred = bcs::from_bytes(&event.contents)
            .expect("Failed to deserialze OwnerCap Transferred event");

        let occurred_at = DateTime::from_timestamp_millis(meta.checkpoint_timestamp_ms())
            .expect("Failed to parse checkpoint timestamp into DateTime");

        Self {
            occurred_at,
            id: move_event.owner_cap_id.to_hex(),
            object_id: move_event.authorized_object_id.to_hex(),
            previous_owner: move_event.previous_owner.to_hex(),
            owner: move_event.owner.to_hex(),
        }
    }
}
