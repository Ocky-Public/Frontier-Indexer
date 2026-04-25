use anyhow::Context;
use clap::Parser;
use prometheus::Registry;
use std::path::Path;
use url::Url;

use diesel_async::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use sui_indexer_alt_framework::ingestion::ingestion_client::IngestionClientArgs;
use sui_indexer_alt_framework::ingestion::streaming_client::StreamingClientArgs;
use sui_indexer_alt_framework::ingestion::{ClientArgs, IngestionConfig};
use sui_indexer_alt_framework::pipeline::sequential::SequentialConfig;
use sui_indexer_alt_framework::pipeline::CommitterConfig;
use sui_indexer_alt_framework::{Indexer, IndexerArgs};
use sui_indexer_alt_metrics::db::DbConnectionStatsCollector;
use sui_indexer_alt_metrics::{MetricsArgs, MetricsService};
use sui_pg_db::{Db, DbArgs};

use indexer::handlers::*;
use indexer::models::system::FuelRegistry;
use indexer::models::system::TableRegistry;
use indexer::TESTNET_REMOTE_STORE_URL;
use indexer::{AppContext, AppEnv};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub mod config;
pub mod pipelines;

pub use config::*;
pub use pipelines::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let AppConfig {
        metrics_address,
        db_config,
        indexer,
        sequential,
        ingestion,
        network,
        packages,
        sandbox,
    } = AppConfig::parse();

    let DbConfig {
        db_user,
        db_password,
        db_host,
        db_port,
        db_name,
        db_schema,
        db_connection_pool_size,
        db_connection_timeout_ms,
        db_statement_timeout_ms,
        tls_verify_cert,
        tls_ca_cert_path,
    } = db_config;

    let database_str = format!(
        "postgres://{}:{}@{}:{}/{}?options=-csearch_path%3D{}",
        db_user, db_password, db_host, db_port, db_name, db_schema
    );

    let database_url = Url::parse(&database_str).expect("Failed to construct valid Database URL");

    let db_args = DbArgs {
        db_connection_pool_size,
        db_connection_timeout_ms,
        db_statement_timeout_ms,
        tls_verify_cert,
        tls_ca_cert_path,
    };

    let IndexerConfig {
        first_checkpoint,
        last_checkpoint,
        pipeline,
    } = indexer;

    let pipelines: Vec<String> = if !pipeline.is_empty() {
        pipeline
    } else {
        let config = PipelineConfig::from_file(Path::new("./pipelines.toml"))?;
        config.enabled_pipelines().unwrap_or_default()
    };

    let indexer_args = IndexerArgs {
        first_checkpoint,
        last_checkpoint,
        pipeline: pipelines,
        ..Default::default()
    };

    let streaming_args = StreamingClientArgs::default();

    let Sequential {
        min_eager_rows,
        checkpoint_lag,
        max_batch_checkpoints,
        processor_channel_size,
        write_concurrency,
        collect_interval_ms,
        watermark_interval_ms,
        watermark_interval_jitter_ms,
    } = sequential;

    let sequential = SequentialConfig {
        min_eager_rows,
        checkpoint_lag,
        max_batch_checkpoints,
        processor_channel_size,
        committer: CommitterConfig {
            write_concurrency,
            collect_interval_ms,
            watermark_interval_ms,
            watermark_interval_jitter_ms,
        },
        ..Default::default()
    };

    let Ingestion {
        checkpoint_buffer_size,
        retry_interval_ms,
        streaming_backoff_initial_batch_size,
        streaming_backoff_max_batch_size,
        streaming_connection_timeout_ms,
        streaming_statement_timeout_ms,
    } = ingestion;

    let ingestion = IngestionConfig {
        checkpoint_buffer_size,
        retry_interval_ms,
        streaming_backoff_initial_batch_size,
        streaming_backoff_max_batch_size,
        streaming_connection_timeout_ms,
        streaming_statement_timeout_ms,
        ..Default::default()
    };

    let (env, ingestion_args, packages) = if sandbox.enabled {
        // Sandbox mode - override package addresses then pick ingestion source
        let has_world = !sandbox.world_packages.is_empty();

        indexer::sandbox::init_package_override(sandbox.app_package_ids, sandbox.world_packages);

        let ingestion = match sandbox.env {
            SandboxEnv::Localnet => IngestionClientArgs {
                local_ingestion_path: Some(
                    sandbox
                        .local_ingestion_path
                        .context("--local-ingestion-path is required for localnet")?,
                ),
                ..Default::default()
            },
            SandboxEnv::Testnet => IngestionClientArgs {
                remote_store_url: Some(
                    Url::parse(TESTNET_REMOTE_STORE_URL).expect("invalid testnet remote store URL"),
                ),
                ..Default::default()
            },
        };

        let mut packages = packages;

        if !has_world {
            packages.retain(|p| matches!(p, Package::App));
        }

        let app_env = match sandbox.env {
            SandboxEnv::Testnet | SandboxEnv::Localnet => AppEnv::Testnet,
        };

        (app_env, ingestion, packages)
    } else {
        let env = network.context("network is required when not using sandbox")?;
        let ingestion = IngestionClientArgs {
            remote_store_url: Some(env.remote_store_url()),
            ..Default::default()
        };

        (env, ingestion, packages)
    };

    let registry = Registry::new_custom(Some("frontier".into()), None)
        .context("Failed to create Prometheus registry.")?;

    let metrics = MetricsService::new(MetricsArgs { metrics_address }, registry.clone());

    let store = Db::for_write(database_url, db_args)
        .await
        .context("Failed to connect to database")?;

    let mut conn = store
        .connect()
        .await
        .context("Failed to get connection for database setup")?;

    let schema_name = db_schema.replace('"', "\"\"");
    diesel::sql_query(format!("CREATE SCHEMA IF NOT EXISTS \"{schema_name}\""))
        .execute(&mut *conn)
        .await
        .context("Failed to create database schema")?;

    store
        .run_migrations(Some(&MIGRATIONS))
        .await
        .context("Failed to run pending migrations.")?;

    let table_registry = TableRegistry::load_from_db(&mut conn).await;
    let fuel_registry = FuelRegistry::load_from_db(&mut conn).await;

    let context = AppContext::new(env, table_registry, fuel_registry);

    registry.register(Box::new(DbConnectionStatsCollector::new(
        Some("frontier_indexer_db"),
        store.clone(),
    )))?;

    let client_args = ClientArgs {
        ingestion: ingestion_args,
        streaming: streaming_args,
    };

    let mut indexer = Indexer::new(
        store.clone(),
        indexer_args,
        client_args,
        ingestion,
        None,
        metrics.registry(),
    )
    .await?;

    // Register handlers based on selected packages
    for package in &packages {
        match package {
            #[rustfmt::skip]
            Package::App => {}

            #[rustfmt::skip]
            Package::World => {
                // Owner Caps
                indexer.sequential_pipeline(world::OwnerCapCreatedHandler::new(&context, vec![]), sequential.clone()).await?;
                indexer.sequential_pipeline(world::OwnerCapHandler::new(&context, vec![]), sequential.clone()).await?;
                indexer.sequential_pipeline(world::OwnerCapTransferredHandler::new(&context, vec![]), sequential.clone()).await?;

                // Assemblies
                indexer.sequential_pipeline(world::AssemblyCreatedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::AssemblyHandler::new(&context), sequential.clone()).await?;

                // Extensions
                indexer.sequential_pipeline(world::ExtensionFrozenHandler::new(&context), sequential.clone()).await?;

                // Gates
                indexer.sequential_pipeline(world::GateConfigHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GateCreatedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GateExtensionAuthorizedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GateExtensionRevokedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GateHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GateJumpedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GateLinkedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GateUnlinkedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GatePermitHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::GatePermitIssuedHandler::new(&context), sequential.clone()).await?;

                // Network Nodes
                indexer.sequential_pipeline(world::NetworkNodeCreatedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::NetworkNodeHandler::new(&context), sequential.clone()).await?;

                // Storage Units
                indexer.sequential_pipeline(world::StorageUnitCreatedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::StorageUnitExtensionAuthorizedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::StorageUnitExtensionRevokedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::StorageUnitHandler::new(&context), sequential.clone()).await?;

                // Turrets
                indexer.sequential_pipeline(world::TurretCreatedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::TurretExtensionAuthorizedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::TurretExtensionRevokedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::TurretHandler::new(&context), sequential.clone()).await?;

                // Characters
                indexer.sequential_pipeline(world::CharacterCreatedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::CharacterHandler::new(&context), sequential.clone()).await?;

                // Killmails
                indexer.sequential_pipeline(world::KillmailHandler::new(&context), sequential.clone()).await?;

                // Energy
                indexer.sequential_pipeline(world::EnergyConfigHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::EnergyProductionStartedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::EnergyProductionStoppedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::EnergyReleasedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::EnergyReservedHandler::new(&context), sequential.clone()).await?;

                // Fuel
                indexer.sequential_pipeline(world::FuelBurningStartedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::FuelBurningStoppedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::FuelBurningUpdatedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::FuelConfigHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::FuelDeletedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::FuelDepositedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::FuelEfficiencyRemovedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::FuelEfficiencySetHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::FuelWithdrawnHandler::new(&context), sequential.clone()).await?;

                // Inventories
                indexer.sequential_pipeline(world::InventoryHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::ItemBurnedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::ItemDepositedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::ItemDestroyedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::ItemHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::ItemMintedHandler::new(&context), sequential.clone()).await?;
                indexer.sequential_pipeline(world::ItemWithdrawnHandler::new(&context), sequential.clone()).await?;

                // Locations
                indexer.sequential_pipeline(world::LocationRevealedHandler::new(&context), sequential.clone()).await?;

                // Status
                indexer.sequential_pipeline(world::StatusChangedHandler::new(&context), sequential.clone()).await?;
            }
        }
    }

    let s_indexer = indexer.run().await?;
    let s_metrics = metrics.run().await?;

    s_indexer.attach(s_metrics).main().await?;
    Ok(())
}
