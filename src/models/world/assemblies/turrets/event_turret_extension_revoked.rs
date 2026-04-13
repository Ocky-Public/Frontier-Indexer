use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;

use sui_indexer_alt_framework::FieldCount;
use sui_sdk_types::Address;
use sui_types::event::Event;

use crate::handlers::EventMeta;
use crate::models::world::MoveTenantItemId;
use crate::models::MoveTypeName;
use crate::schema::indexer::events_turret_extension_revoked;

#[derive(Deserialize)]
pub struct MoveTurretExtensionRevoked {
    assembly_id: Address,
    assembly_key: MoveTenantItemId,
    revoked_extension: MoveTypeName,
    owner_cap_id: Address,
}

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = events_turret_extension_revoked)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoredTurretExtensionRevoked {
    event_id: String,
    occurred_at: DateTime<Utc>,
    id: String,
    item_id: String,
    package_id: String,
    module_name: String,
    struct_name: String,
}

impl StoredTurretExtensionRevoked {
    pub fn from_event(event: &Event, meta: &EventMeta) -> Self {
        let move_event: MoveTurretExtensionRevoked = bcs::from_bytes(&event.contents)
            .expect("Failed to deserialze Turret Extension Revoked event");

        let occurred_at = DateTime::from_timestamp_millis(meta.checkpoint_timestamp_ms())
            .expect("Failed ot parse checkpoint timestamp into DateTime");

        let (package_id, module_name, struct_name) = move_event.revoked_extension.to_components();

        Self {
            event_id: meta.event_digest(),
            occurred_at,
            id: move_event.assembly_id.to_hex(),
            item_id: move_event.assembly_key.item_id.to_string(),
            package_id,
            module_name,
            struct_name,
        }
    }
}
