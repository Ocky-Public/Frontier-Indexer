use anyhow::Context;
use clap::Parser;
use prometheus::Registry;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio;
use url::Url;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use sui_indexer_alt_framework::ingestion::ingestion_client::IngestionClientArgs;
use sui_indexer_alt_framework::ingestion::streaming_client::StreamingClientArgs;
use sui_indexer_alt_framework::ingestion::{ClientArgs, IngestionConfig};
use sui_indexer_alt_framework::{Indexer, IndexerArgs};
use sui_indexer_alt_metrics::db::DbConnectionStatsCollector;
use sui_indexer_alt_metrics::{MetricsArgs, MetricsService};
use sui_pg_db::{Db, DbArgs};

use indexer::handlers::*;
use indexer::{AppEnv, TESTNET_REMOTE_STORE_URL};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Package {
    /// Index your own application data.
    App,

    /// Index the frontier world data.
    World,
}

#[derive(Parser)]
struct DbConfig {
    #[arg(long, env = "DB_USER", default_value = "postgres")]
    pub db_user: String,

    #[arg(long, env = "DB_PASSWORD", default_value = "postgres")]
    pub db_password: String,

    #[arg(long, env = "DB_HOST", default_value = "localhost")]
    pub db_host: String,

    #[arg(long, env = "DB_PORT", default_value_t = 5432)]
    pub db_port: u16,

    #[arg(long, env = "DB_NAME", default_value = "postgres")]
    pub db_name: String,

    #[arg(long, env = "DB_SCHEMA", default_value = "indexer")]
    pub db_schema: String,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum SandboxEnv {
    Testnet,
    Localnet,
}

#[derive(Parser)]
struct SandboxArgs {
    #[arg(
        long,
        env = "SANDBOX",
        requires = "app_package_id",
        default_value_t = false
    )]
    pub enabled: bool,

    #[arg(long, env = "SANDBOX_NETWORK", default_value = "localnet")]
    env: SandboxEnv,

    #[arg(long, env = "SANDBOX_APP_PACKAGES", value_delimiter = ',')]
    pub app_package_id: Vec<String>,

    #[clap(long, env = "SANDBOX_WORLD_PACKAGES", value_delimiter = ',')]
    pub world_packages: Vec<String>,

    #[clap(long, env = "SANDBOX_INGESTION_PATH")]
    pub local_ingestion_path: Option<PathBuf>,
}

#[derive(Parser)]
struct AppConfig {
    #[command(flatten)]
    pub db_config: DbConfig,

    #[arg(long, env = "SUI_NETWORK", default_value = "testnet")]
    pub network: Option<AppEnv>,

    #[arg(long, env = "PACKAGES", value_enum, default_values = ["app", "world"], value_delimiter = ',')]
    pub packages: Vec<Package>,

    #[arg(long, env = "METRICS_ADDRESS", default_value = "0.0.0.0:9184")]
    pub metrics_address: SocketAddr,

    #[arg(long, env = "FIRST_CHECKPOINT")]
    pub first_checkpoint: Option<u64>,

    #[arg(long, env = "LAST_CHECKPOINT")]
    pub last_checkpoint: Option<u64>,

    #[command(flatten)]
    pub sandbox: SandboxArgs,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let AppConfig {
        metrics_address,
        db_config,
        network,
        packages,
        first_checkpoint,
        last_checkpoint,
        sandbox,
    } = AppConfig::parse();

    let database_str = format!(
        "postgres://{}:{}@{}:{}/{}?options=-csearch_path%3D{}",
        db_config.db_user,
        db_config.db_password,
        db_config.db_host,
        db_config.db_port,
        db_config.db_name,
        db_config.db_schema
    );

    let database_url = Url::parse(&database_str).expect("Failed to construct valid Database URL");

    let db_args = DbArgs {
        ..Default::default()
    };

    let indexer_args = IndexerArgs {
        first_checkpoint,
        last_checkpoint,
        ..Default::default()
    };

    let streaming_args = StreamingClientArgs {
        ..Default::default()
    };

    let (env, ingestion_args, packages) = if sandbox.enabled {
        // Sandbox mode - override package addresses then pick ingenstion source
        let has_world = !sandbox.world_packages.is_empty();

        indexer::sandbox::init_package_override(sandbox.app_package_id, sandbox.world_packages);

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

    let registry = Registry::new_custom(Some("frontier_indexer".into()), None)
        .context("Failt to create Prometheus registry.")?;

    let metrics = MetricsService::new(MetricsArgs { metrics_address }, registry.clone());

    // Prepare store for the indexer
    let store = Db::for_write(database_url, db_args)
        .await
        .context("Failed to connect to database")?;

    store
        .run_migrations(Some(&MIGRATIONS))
        .await
        .context("Failed to run pending migrations.")?;

    registry.register(Box::new(DbConnectionStatsCollector::new(
        Some("frontier_indexer_db"),
        store.clone(),
    )))?;

    let mut indexer = Indexer::new(
        store,
        indexer_args,
        ClientArgs {
            ingestion: ingestion_args,
            streaming: streaming_args,
        },
        IngestionConfig::default(),
        None,
        metrics.registry(),
    )
    .await?;

    // Register handlers based on selected packages
    for package in &packages {
        match package {
            Package::App => {}
            Package::World => {
                indexer
                    .sequential_pipeline(
                        world::OwnerCapCreatedHandler::new(env),
                        Default::default(),
                    )
                    .await?;

                indexer
                    .sequential_pipeline(
                        world::OwnerCapTransferredHandler::new(env),
                        Default::default(),
                    )
                    .await?;

                indexer
                    .sequential_pipeline(
                        world::OwnerCapHandler::new(env),
                        Default::default()
                    )
                    .await?;

                indexer
                    .sequential_pipeline(
                        world::AssemblyHandler::new(env),
                        Default::default()
                    )
                    .await?;

                indexer
                    .sequential_pipeline(
                        world::AssemblyCreatedHandler::new(env),
                        Default::default(),
                    )
                    .await?;

                indexer
                    .sequential_pipeline(
                        world::CharacterHandler::new(env),
                        Default::default()
                    )
                    .await?;

                indexer
                    .sequential_pipeline(
                        world::CharacterCreatedHandler::new(env),
                        Default::default(),
                    )
                    .await?;

                indexer
                    .sequential_pipeline(
                        world::LocationRevealedHandler::new(env),
                        Default::default(),
                    )
                    .await?;

                indexer
                    .sequential_pipeline(
                        world::StatusChangedHandler::new(env),
                        Default::default()
                    )
                    .await?;
            }
        }
    }

    let s_indexer = indexer.run().await?;
    let s_metrics = metrics.run().await?;

    s_indexer.attach(s_metrics).main().await?;
    Ok(())
}
