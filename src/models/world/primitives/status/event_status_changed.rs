use chrono::DateTime;
use diesel::prelude::*;
use serde::Deserialize;

use sui_indexer_alt_framework::FieldCount;
use sui_sdk_types::Address;
use sui_types::event::Event;

use crate::handlers::EventMeta;
use crate::models::world::MoveAction;
use crate::models::world::MoveStatus;
use crate::models::world::MoveTenantItemId;
use crate::schema::indexer::events_status_changed;

#[derive(Deserialize)]
pub struct MoveStatusChanged {
    assembly_id: Address,
    assembly_key: MoveTenantItemId,
    status: MoveStatus,
    action: MoveAction,
}

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = events_status_changed)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoredStatusChanged {
    pub event_id: String,
    pub occurred_at: DateTime<chrono::Utc>,
    pub id: String,
    pub item_id: String,
    pub tenant: String,
    pub status: String,
    pub action: String,
}

impl StoredStatusChanged {
    pub fn from_event(event: &Event, meta: &EventMeta) -> Self {
        let move_event: MoveStatusChanged =
            bcs::from_bytes(&event.contents).expect("Failed to deserialize Status Changed event");

        let occurred_at = DateTime::from_timestamp_millis(meta.checkpoint_timestamp_ms())
            .expect("Failed to parse checkpoint timestamp into DateTime");

        Self {
            event_id: meta.event_digest(),
            occurred_at,
            id: move_event.assembly_id.to_hex(),
            item_id: move_event.assembly_key.item_id.to_string(),
            tenant: move_event.assembly_key.tenant.to_string(),
            status: move_event.status.as_ref().to_string(),
            action: move_event.action.as_ref().to_string(),
        }
    }
}
