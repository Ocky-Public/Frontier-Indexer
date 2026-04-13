use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use strum_macros::{AsRefStr, Display, EnumString};

use sui_indexer_alt_framework::FieldCount;
use sui_sdk_types::Address;
use sui_types::object::Object;

use crate::models::world::MoveTenantItemId;
use crate::schema::indexer::killmails;

#[derive(Deserialize, Debug, Clone, Copy, Display, EnumString, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MoveLossType {
    Ship = 0,
    Structure = 1,
}

#[derive(Deserialize)]
pub struct MoveKillmail {
    id: Address,
    key: MoveTenantItemId,
    killer_id: MoveTenantItemId,
    victim_id: MoveTenantItemId,
    reported_by_character_id: MoveTenantItemId,
    kill_timestamp: u64,
    loss_type: MoveLossType,
    solar_system_id: MoveTenantItemId,
}

#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = killmails)]
pub struct StoredKillmail {
    id: String,
    kill_id: String,
    tenant: String,
    occurred_at: DateTime<Utc>,
    solar_system_id: String,
    loss_type: String,
    killer_id: String,
    victim_id: String,
    reporter_id: String,
}

impl StoredKillmail {
    pub fn from_object(obj: &Object) -> Self {
        let move_obj = obj.data.try_as_move().expect("Object is not a Move object");
        let bytes = move_obj.contents();

        let killmail: MoveKillmail =
            bcs::from_bytes(bytes).expect("Failed to deserial Killmail object");

        let occurred_at = DateTime::from_timestamp(killmail.kill_timestamp as i64, 0)
            .expect("Failed to parse killmail timestamp into DateTime");

        Self {
            id: killmail.id.to_hex(),
            kill_id: killmail.key.item_id.to_string(),
            tenant: killmail.key.tenant,
            occurred_at,
            solar_system_id: killmail.solar_system_id.item_id.to_string(),
            loss_type: killmail.loss_type.as_ref().to_string(),
            killer_id: killmail.killer_id.item_id.to_string(),
            victim_id: killmail.victim_id.item_id.to_string(),
            reporter_id: killmail.reported_by_character_id.item_id.to_string(),
        }
    }
}
