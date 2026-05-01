use async_trait::async_trait;
use serde::Serialize;

use crate::handlers::world::*;
use crate::models::world::*;
use crate::transports::Routing;

pub struct RedisTransport {
    id: String,
    manager: redis::aio::ConnectionManager,
    channel_prefix: String,
}

impl RedisTransport {
    pub async fn connect(
        id: &str,
        url: &str,
        channel_prefix: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let client = redis::Client::open(url)?;
        let manager = redis::aio::ConnectionManager::new(client).await?;

        Ok(Self {
            id: id.to_string(),
            manager,
            channel_prefix: channel_prefix.into(),
        })
    }

    async fn send<I: Serialize + Send + Sync + 'static>(
        &self,
        channel: String,
        item: &I,
    ) -> anyhow::Result<()> {
        let channel = format!("{}:{}", self.channel_prefix, channel);
        let payload = serde_json::to_string(item)?;
        let mut conn = self.manager.clone();
        redis::AsyncCommands::publish::<_, _, ()>(&mut conn, &channel, &payload).await?;
        Ok(())
    }
}

// Owner Caps
#[async_trait]
impl Routing<StoredOwnerCapCreated> for RedisTransport {
    async fn send(
        &self,
        _pipeline: &'static str,
        item: &StoredOwnerCapCreated,
    ) -> anyhow::Result<()> {
        let channel = format!("{}:{}:{}", item.object_id, "owner_cap", "created");
        self.send(channel, item).await
    }
}

#[async_trait]
impl Routing<OwnerCapAction> for RedisTransport {
    async fn send(&self, _pipeline: &'static str, action: &OwnerCapAction) -> anyhow::Result<()> {
        match action {
            OwnerCapAction::Upsert(item) => {
                let channel = format!("{}:{}:{}", item.object_id, "owner_cap", "updated");
                self.send(channel, item).await
            }
            OwnerCapAction::Delete(id_str) => {
                let channel = format!("{}:{}:{}", id_str, "owner_cap", "deleted");
                self.send(channel, id_str).await
            }
        }
    }
}

#[async_trait]
impl Routing<StoredOwnerCapTransferred> for RedisTransport {
    async fn send(
        &self,
        _pipeline: &'static str,
        item: &StoredOwnerCapTransferred,
    ) -> anyhow::Result<()> {
        let channel = format!("{}:{}:{}", item.id, "owner_cap", "transferred");
        self.send(channel, item).await
    }
}

// Assemblies
#[async_trait]
impl Routing<StoredAssemblyCreated> for RedisTransport {
    async fn send(
        &self,
        _pipeline: &'static str,
        item: &StoredAssemblyCreated,
    ) -> anyhow::Result<()> {
        let channel = format!("{}:{}:{}", item.id, "assembly", "created");
        self.send(channel, item).await
    }
}

#[async_trait]
impl Routing<AssemblyAction> for RedisTransport {
    async fn send(&self, _pipeline: &'static str, action: &AssemblyAction) -> anyhow::Result<()> {
        match action {
            AssemblyAction::Upsert(item) => {
                let channel = format!("{}:{}:{}", item.id, "assembly", "updated");
                self.send(channel, item).await
            }
            AssemblyAction::Delete(id_str) => {
                let channel = format!("{}:{}:{}", id_str, "assembly", "deleted");
                self.send(channel, id_str).await
            }
        }
    }
}

// Extensions
#[async_trait]
impl Routing<StoredExtensionFrozen> for RedisTransport {
    async fn send(
        &self,
        _pipeline: &'static str,
        item: &StoredExtensionFrozen,
    ) -> anyhow::Result<()> {
        let channel = format!("{}:{}:{}", item.id, "extension", "frozen");
        self.send(channel, item).await
    }
}

// Gates
impl Routing<GateConfigAction> for RedisTransport {}
impl Routing<StoredGateCreated> for RedisTransport {}
impl Routing<StoredGateExtensionAuthorized> for RedisTransport {}
impl Routing<StoredGateExtensionRevoked> for RedisTransport {}
impl Routing<GateAction> for RedisTransport {}
impl Routing<StoredGateJumped> for RedisTransport {}
impl Routing<StoredGateLinked> for RedisTransport {}
impl Routing<GatePermitAction> for RedisTransport {}
impl Routing<StoredGatePermitIssued> for RedisTransport {}
impl Routing<StoredGateUnlinked> for RedisTransport {}

// Network Nodes
impl Routing<StoredNetworkNodeCreated> for RedisTransport {}
impl Routing<NetworkNodeAction> for RedisTransport {}

// Storage Units
impl Routing<StoredStorageUnitCreated> for RedisTransport {}
impl Routing<StoredStorageUnitExtensionAuthorized> for RedisTransport {}
impl Routing<StoredStorageUnitExtensionRevoked> for RedisTransport {}
impl Routing<StorageUnitAction> for RedisTransport {}

// Turrets
impl Routing<StoredTurretCreated> for RedisTransport {}
impl Routing<StoredTurretExtensionAuthorized> for RedisTransport {}
impl Routing<StoredTurretExtensionRevoked> for RedisTransport {}
impl Routing<TurretAction> for RedisTransport {}

// Characters
impl Routing<StoredCharacterCreated> for RedisTransport {}
impl Routing<CharacterAction> for RedisTransport {}

// Killmails
impl Routing<StoredKillmail> for RedisTransport {}

// Energy
impl Routing<EnergyConfigAction> for RedisTransport {}
impl Routing<StoredEnergyProductionStarted> for RedisTransport {}
impl Routing<StoredEnergyProductionStopped> for RedisTransport {}
impl Routing<StoredEnergyReleased> for RedisTransport {}
impl Routing<StoredEnergyReserved> for RedisTransport {}

// Fuel
impl Routing<StoredFuelBurningStarted> for RedisTransport {}
impl Routing<StoredFuelBurningStopped> for RedisTransport {}
impl Routing<StoredFuelBurningUpdated> for RedisTransport {}
impl Routing<FuelConfigAction> for RedisTransport {}
impl Routing<StoredFuelDeleted> for RedisTransport {}
impl Routing<StoredFuelDeposited> for RedisTransport {}
impl Routing<StoredFuelEfficiencyRemoved> for RedisTransport {}
impl Routing<StoredFuelEfficiencySet> for RedisTransport {}
impl Routing<StoredFuelWithdrawn> for RedisTransport {}

// Inventories
impl Routing<InventoryAction> for RedisTransport {}
impl Routing<StoredItemBurned> for RedisTransport {}
impl Routing<StoredItemDeposited> for RedisTransport {}
impl Routing<StoredItemDestroyed> for RedisTransport {}
impl Routing<ItemAction> for RedisTransport {}
impl Routing<StoredItemMinted> for RedisTransport {}
impl Routing<StoredItemWithdrawn> for RedisTransport {}

// Locations
impl Routing<StoredLocationRevealed> for RedisTransport {}

// Status
impl Routing<StoredStatusChanged> for RedisTransport {}
