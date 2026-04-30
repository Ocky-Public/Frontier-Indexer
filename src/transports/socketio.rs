use async_trait::async_trait;
use serde::Serialize;
use socketioxide::SocketIo;

use crate::handlers::world::*;
use crate::models::world::*;
use crate::transports::Routing;

pub struct SocketIoTransport {
    id: String,
    io: SocketIo,
}

impl SocketIoTransport {
    pub fn new(id: &str, io: SocketIo) -> Self {
        Self {
            id: id.to_string(),
            io,
        }
    }

    async fn send<I: Serialize + Send + Sync + 'static>(
        &self,
        room: String,
        event: String,
        item: &I,
    ) -> anyhow::Result<()> {
        let _ = self.io.to(room).emit(event, item);
        Ok(())
    }
}

// Owner Caps
impl Routing<StoredOwnerCapCreated> for SocketIoTransport {}
impl Routing<OwnerCapAction> for SocketIoTransport {}
impl Routing<StoredOwnerCapTransferred> for SocketIoTransport {}

// Assemblies
impl Routing<StoredAssemblyCreated> for SocketIoTransport {}
impl Routing<AssemblyAction> for SocketIoTransport {}

// Extensions
impl Routing<StoredExtensionFrozen> for SocketIoTransport {}

// Gates
impl Routing<GateConfigAction> for SocketIoTransport {}
impl Routing<StoredGateCreated> for SocketIoTransport {}
impl Routing<StoredGateExtensionAuthorized> for SocketIoTransport {}
impl Routing<StoredGateExtensionRevoked> for SocketIoTransport {}
impl Routing<GateAction> for SocketIoTransport {}
impl Routing<StoredGateJumped> for SocketIoTransport {}
impl Routing<StoredGateLinked> for SocketIoTransport {}
impl Routing<GatePermitAction> for SocketIoTransport {}
impl Routing<StoredGatePermitIssued> for SocketIoTransport {}
impl Routing<StoredGateUnlinked> for SocketIoTransport {}

// Network Nodes
impl Routing<StoredNetworkNodeCreated> for SocketIoTransport {}
impl Routing<NetworkNodeAction> for SocketIoTransport {}

// Storage Units
impl Routing<StoredStorageUnitCreated> for SocketIoTransport {}
impl Routing<StoredStorageUnitExtensionAuthorized> for SocketIoTransport {}
impl Routing<StoredStorageUnitExtensionRevoked> for SocketIoTransport {}
impl Routing<StorageUnitAction> for SocketIoTransport {}

// Turrets
impl Routing<StoredTurretCreated> for SocketIoTransport {}
impl Routing<StoredTurretExtensionAuthorized> for SocketIoTransport {}
impl Routing<StoredTurretExtensionRevoked> for SocketIoTransport {}
impl Routing<TurretAction> for SocketIoTransport {}

// Characters
impl Routing<StoredCharacterCreated> for SocketIoTransport {}
impl Routing<CharacterAction> for SocketIoTransport {}

// Killmails
impl Routing<StoredKillmail> for SocketIoTransport {}

// Energy
impl Routing<EnergyConfigAction> for SocketIoTransport {}
impl Routing<StoredEnergyProductionStarted> for SocketIoTransport {}
impl Routing<StoredEnergyProductionStopped> for SocketIoTransport {}
impl Routing<StoredEnergyReleased> for SocketIoTransport {}
impl Routing<StoredEnergyReserved> for SocketIoTransport {}

// Fuel
impl Routing<StoredFuelBurningStarted> for SocketIoTransport {}
impl Routing<StoredFuelBurningStopped> for SocketIoTransport {}
impl Routing<StoredFuelBurningUpdated> for SocketIoTransport {}
impl Routing<FuelConfigAction> for SocketIoTransport {}
impl Routing<StoredFuelDeleted> for SocketIoTransport {}
impl Routing<StoredFuelDeposited> for SocketIoTransport {}
impl Routing<StoredFuelEfficiencyRemoved> for SocketIoTransport {}
impl Routing<StoredFuelEfficiencySet> for SocketIoTransport {}
impl Routing<StoredFuelWithdrawn> for SocketIoTransport {}

// Inventories
impl Routing<InventoryAction> for SocketIoTransport {}
impl Routing<StoredItemBurned> for SocketIoTransport {}
impl Routing<StoredItemDeposited> for SocketIoTransport {}
impl Routing<StoredItemDestroyed> for SocketIoTransport {}
impl Routing<ItemAction> for SocketIoTransport {}
impl Routing<StoredItemMinted> for SocketIoTransport {}
impl Routing<StoredItemWithdrawn> for SocketIoTransport {}

// Locations
impl Routing<StoredLocationRevealed> for SocketIoTransport {}

// Status
impl Routing<StoredStatusChanged> for SocketIoTransport {}
