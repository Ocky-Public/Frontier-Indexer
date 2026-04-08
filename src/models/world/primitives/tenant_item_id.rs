use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveTenantItemId {
    pub item_id: u64,
    pub tenant: String,
}
