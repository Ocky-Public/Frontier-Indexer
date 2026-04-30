use async_trait::async_trait;
use serde::Serialize;

use crate::handlers::world::*;
use crate::models::world::*;
use crate::transports::Routing;

pub struct AmqpTransport {
    id: String,
    pool: deadpool_lapin::Pool,
    exchange: String,
}

impl AmqpTransport {
    pub async fn connect(
        id: &str,
        addr: &str,
        exchange: impl Into<String>,
        pool_size: usize,
    ) -> anyhow::Result<Self> {
        let exchange = exchange.into();
        let mut cfg = deadpool_lapin::Config::default();

        cfg.url = Some(addr.to_string());
        cfg.pool = Some(deadpool_lapin::PoolConfig {
            max_size: pool_size,
            ..Default::default()
        });

        let pool = cfg.create_pool(Some(deadpool_lapin::Runtime::Tokio1))?;
        {
            let conn = pool.get().await?;
            let channel = conn.create_channel().await?;
            channel
                .exchange_declare(
                    &exchange,
                    lapin::ExchangeKind::Topic,
                    lapin::options::ExchangeDeclareOptions {
                        durable: true,
                        ..Default::default()
                    },
                    lapin::types::FieldTable::default(),
                )
                .await?;
        }

        Ok(Self {
            id: id.to_string(),
            pool,
            exchange,
        })
    }

    async fn send<I: Serialize + Send + Sync + 'static>(
        &self,
        routing: String,
        item: &I,
    ) -> anyhow::Result<()> {
        let amqp_key = format!("indexer.{}", routing);
        let payload = serde_json::to_vec(item)?;
        let conn = self.pool.get().await?;
        let channel = conn.create_channel().await?;

        channel
            .basic_publish(
                &self.exchange,
                &amqp_key,
                lapin::options::BasicPublishOptions::default(),
                &payload,
                lapin::BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2),
            )
            .await?
            .await?; // second await = broker publisher-confirm ack

        Ok(())
    }
}

// Owner Caps
impl Routing<StoredOwnerCapCreated> for AmqpTransport {}
impl Routing<OwnerCapAction> for AmqpTransport {}
impl Routing<StoredOwnerCapTransferred> for AmqpTransport {}

// Assemblies
impl Routing<StoredAssemblyCreated> for AmqpTransport {}
impl Routing<AssemblyAction> for AmqpTransport {}

// Extensions
impl Routing<StoredExtensionFrozen> for AmqpTransport {}

// Gates
impl Routing<GateConfigAction> for AmqpTransport {}
impl Routing<StoredGateCreated> for AmqpTransport {}
impl Routing<StoredGateExtensionAuthorized> for AmqpTransport {}
impl Routing<StoredGateExtensionRevoked> for AmqpTransport {}
impl Routing<GateAction> for AmqpTransport {}
impl Routing<StoredGateJumped> for AmqpTransport {}
impl Routing<StoredGateLinked> for AmqpTransport {}
impl Routing<GatePermitAction> for AmqpTransport {}
impl Routing<StoredGatePermitIssued> for AmqpTransport {}
impl Routing<StoredGateUnlinked> for AmqpTransport {}

// Network Nodes
impl Routing<StoredNetworkNodeCreated> for AmqpTransport {}
impl Routing<NetworkNodeAction> for AmqpTransport {}

// Storage Units
impl Routing<StoredStorageUnitCreated> for AmqpTransport {}
impl Routing<StoredStorageUnitExtensionAuthorized> for AmqpTransport {}
impl Routing<StoredStorageUnitExtensionRevoked> for AmqpTransport {}
impl Routing<StorageUnitAction> for AmqpTransport {}

// Turrets
impl Routing<StoredTurretCreated> for AmqpTransport {}
impl Routing<StoredTurretExtensionAuthorized> for AmqpTransport {}
impl Routing<StoredTurretExtensionRevoked> for AmqpTransport {}
impl Routing<TurretAction> for AmqpTransport {}

// Characters
impl Routing<StoredCharacterCreated> for AmqpTransport {}
impl Routing<CharacterAction> for AmqpTransport {}

// Killmails
impl Routing<StoredKillmail> for AmqpTransport {}

// Energy
impl Routing<EnergyConfigAction> for AmqpTransport {}
impl Routing<StoredEnergyProductionStarted> for AmqpTransport {}
impl Routing<StoredEnergyProductionStopped> for AmqpTransport {}
impl Routing<StoredEnergyReleased> for AmqpTransport {}
impl Routing<StoredEnergyReserved> for AmqpTransport {}

// Fuel
impl Routing<StoredFuelBurningStarted> for AmqpTransport {}
impl Routing<StoredFuelBurningStopped> for AmqpTransport {}
impl Routing<StoredFuelBurningUpdated> for AmqpTransport {}
impl Routing<FuelConfigAction> for AmqpTransport {}
impl Routing<StoredFuelDeleted> for AmqpTransport {}
impl Routing<StoredFuelDeposited> for AmqpTransport {}
impl Routing<StoredFuelEfficiencyRemoved> for AmqpTransport {}
impl Routing<StoredFuelEfficiencySet> for AmqpTransport {}
impl Routing<StoredFuelWithdrawn> for AmqpTransport {}

// Inventories
impl Routing<InventoryAction> for AmqpTransport {}
impl Routing<StoredItemBurned> for AmqpTransport {}
impl Routing<StoredItemDeposited> for AmqpTransport {}
impl Routing<StoredItemDestroyed> for AmqpTransport {}
impl Routing<ItemAction> for AmqpTransport {}
impl Routing<StoredItemMinted> for AmqpTransport {}
impl Routing<StoredItemWithdrawn> for AmqpTransport {}

// Locations
impl Routing<StoredLocationRevealed> for AmqpTransport {}

// Status
impl Routing<StoredStatusChanged> for AmqpTransport {}
