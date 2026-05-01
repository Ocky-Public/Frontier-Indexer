use async_nats::Client;
use async_trait::async_trait;
use serde::Serialize;

use crate::handlers::world::*;
use crate::models::world::*;
use crate::transports::Routing;

pub struct NatsTransport {
    id: String,
    client: Client,
    subject_prefix: String,
}

impl NatsTransport {
    pub async fn connect(
        id: &str,
        url: &str,
        subject_prefix: impl Into<String>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            id: id.to_string(),
            client: async_nats::connect(url).await?,
            subject_prefix: subject_prefix.into(),
        })
    }

    async fn send<I: Serialize + Send + Sync + 'static>(
        &self,
        routing: String,
        item: &I,
    ) -> anyhow::Result<()> {
        let subject = format!("{}.{}", self.subject_prefix, routing);
        let payload = serde_json::to_vec(item)?;
        self.client.publish(subject, payload.into()).await?;
        Ok(())
    }
}

// Owner Caps
impl Routing<StoredOwnerCapCreated> for NatsTransport {}
impl Routing<OwnerCapAction> for NatsTransport {}
impl Routing<StoredOwnerCapTransferred> for NatsTransport {}

// Assemblies
impl Routing<StoredAssemblyCreated> for NatsTransport {}
impl Routing<AssemblyAction> for NatsTransport {}

// Extensions
impl Routing<StoredExtensionFrozen> for NatsTransport {}

// Gates
impl Routing<GateConfigAction> for NatsTransport {}
impl Routing<StoredGateCreated> for NatsTransport {}
impl Routing<StoredGateExtensionAuthorized> for NatsTransport {}
impl Routing<StoredGateExtensionRevoked> for NatsTransport {}
impl Routing<GateAction> for NatsTransport {}
impl Routing<StoredGateJumped> for NatsTransport {}
impl Routing<StoredGateLinked> for NatsTransport {}
impl Routing<GatePermitAction> for NatsTransport {}
impl Routing<StoredGatePermitIssued> for NatsTransport {}
impl Routing<StoredGateUnlinked> for NatsTransport {}

// Network Nodes
impl Routing<StoredNetworkNodeCreated> for NatsTransport {}
impl Routing<NetworkNodeAction> for NatsTransport {}

// Storage Units
impl Routing<StoredStorageUnitCreated> for NatsTransport {}
impl Routing<StoredStorageUnitExtensionAuthorized> for NatsTransport {}
impl Routing<StoredStorageUnitExtensionRevoked> for NatsTransport {}
impl Routing<StorageUnitAction> for NatsTransport {}

// Turrets
impl Routing<StoredTurretCreated> for NatsTransport {}
impl Routing<StoredTurretExtensionAuthorized> for NatsTransport {}
impl Routing<StoredTurretExtensionRevoked> for NatsTransport {}
impl Routing<TurretAction> for NatsTransport {}

// Characters
impl Routing<StoredCharacterCreated> for NatsTransport {}
impl Routing<CharacterAction> for NatsTransport {}

// Killmails
impl Routing<StoredKillmail> for NatsTransport {}

// Energy
impl Routing<EnergyConfigAction> for NatsTransport {}
impl Routing<StoredEnergyProductionStarted> for NatsTransport {}
impl Routing<StoredEnergyProductionStopped> for NatsTransport {}
impl Routing<StoredEnergyReleased> for NatsTransport {}
impl Routing<StoredEnergyReserved> for NatsTransport {}

// Fuel
impl Routing<StoredFuelBurningStarted> for NatsTransport {}
impl Routing<StoredFuelBurningStopped> for NatsTransport {}
impl Routing<StoredFuelBurningUpdated> for NatsTransport {}
impl Routing<FuelConfigAction> for NatsTransport {}
impl Routing<StoredFuelDeleted> for NatsTransport {}
impl Routing<StoredFuelDeposited> for NatsTransport {}
impl Routing<StoredFuelEfficiencyRemoved> for NatsTransport {}
impl Routing<StoredFuelEfficiencySet> for NatsTransport {}
impl Routing<StoredFuelWithdrawn> for NatsTransport {}

// Inventories
impl Routing<InventoryAction> for NatsTransport {}
impl Routing<StoredItemBurned> for NatsTransport {}
impl Routing<StoredItemDeposited> for NatsTransport {}
impl Routing<StoredItemDestroyed> for NatsTransport {}
impl Routing<ItemAction> for NatsTransport {}
impl Routing<StoredItemMinted> for NatsTransport {}
impl Routing<StoredItemWithdrawn> for NatsTransport {}

// Locations
impl Routing<StoredLocationRevealed> for NatsTransport {}

// Status
impl Routing<StoredStatusChanged> for NatsTransport {}
