use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;

use indexer::AppEnv;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Package {
    /// Index your own application data.
    App,

    /// Index the frontier world data.
    World,
}

#[derive(Parser)]
pub struct DbConfig {
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

    #[arg(long, env = "DB_CONNECTION_POOL_SIZE", default_value_t = 100)]
    pub db_connection_pool_size: u32,

    #[arg(long, env = "DB_CONNECTION_TIMEOUT_MS", default_value_t = 60_000)]
    pub db_connection_timeout_ms: u64,

    #[arg(long, env = "DB_STATEMENT_TIMEOUT_MS")]
    pub db_statement_timeout_ms: Option<u64>,

    #[arg(long, env = "DB_TLS_VERIFY_CERT", default_value_t = false)]
    pub tls_verify_cert: bool,

    #[arg(long, env = "DB_TLS_CA_CERT_PATH")]
    pub tls_ca_cert_path: Option<PathBuf>,
}

#[derive(Parser)]
pub struct IndexerConfig {
    #[arg(long, env = "FIRST_CHECKPOINT")]
    pub first_checkpoint: Option<u64>,

    #[arg(long, env = "LAST_CHECKPOINT")]
    pub last_checkpoint: Option<u64>,

    #[arg(long, env = "PIPELINES", value_delimiter = ',')]
    pub pipeline: Vec<String>,
}

#[derive(Parser)]
pub struct Sequential {
    #[arg(long, env = "CHECKPOINT_LAG", default_value_t = 0)]
    pub checkpoint_lag: u64,

    #[arg(long, env = "MIN_EAGER_ROWS")]
    pub min_eager_rows: Option<usize>,

    #[arg(long, env = "MAX_BATCH_CHECKPOINTS")]
    pub max_batch_checkpoints: Option<usize>,

    #[arg(long, env = "PROCESSOR_CHANNEL_SIZE")]
    pub processor_channel_size: Option<usize>,

    #[arg(long, env = "WRITE_CONCURRENCY", default_value_t = 5)]
    pub write_concurrency: usize,

    #[arg(long, env = "COLLECT_INTERVAL_MS", default_value_t = 500)]
    pub collect_interval_ms: u64,

    #[arg(long, env = "WATERMARK_INTERVAL_MS", default_value_t = 500)]
    pub watermark_interval_ms: u64,

    #[arg(long, env = "WATERMARK_INTERVAL_JITTER_MS", default_value_t = 0)]
    pub watermark_interval_jitter_ms: u64,
}

#[derive(Parser)]
pub struct Ingestion {
    #[arg(long, env = "CHECKPOINT_BUFFER_SIZE", default_value_t = 50)]
    pub checkpoint_buffer_size: usize,

    #[arg(long, env = "RETRY_INTERVAL_MS", default_value_t = 200)]
    pub retry_interval_ms: u64,

    #[arg(
        long,
        env = "STREAMING_BACKOFF_INITIAL_BATCH_SIZE",
        default_value_t = 10
    )]
    pub streaming_backoff_initial_batch_size: usize,

    #[arg(
        long,
        env = "STREAMING_BACKOFF_MAX_BATCH_SIZE",
        default_value_t = 10000
    )]
    pub streaming_backoff_max_batch_size: usize,

    #[arg(long, env = "STREAMING_CONNECTION_TIMEOUT_MS", default_value_t = 5000)]
    pub streaming_connection_timeout_ms: u64,

    #[arg(long, env = "STREAMING_STATEMENT_TIMEOUT_MS", default_value_t = 5000)]
    pub streaming_statement_timeout_ms: u64,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum SandboxEnv {
    Testnet,
    Localnet,
}

#[derive(Parser)]
pub struct SandboxArgs {
    #[arg(
        long,
        env = "SANDBOX",
        requires = "app_package_id",
        default_value_t = false
    )]
    pub enabled: bool,

    #[arg(long, env = "SANDBOX_NETWORK", default_value = "localnet")]
    pub env: SandboxEnv,

    #[arg(long, env = "SANDBOX_APP_PACKAGES", value_delimiter = ',')]
    pub app_package_ids: Vec<String>,

    #[clap(long, env = "SANDBOX_WORLD_PACKAGES", value_delimiter = ',')]
    pub world_packages: Vec<String>,

    #[clap(long, env = "SANDBOX_INGESTION_PATH")]
    pub local_ingestion_path: Option<PathBuf>,
}

#[derive(Parser)]
pub struct AppConfig {
    #[command(flatten)]
    pub db_config: DbConfig,

    #[command(flatten)]
    pub indexer: IndexerConfig,

    #[command(flatten)]
    pub sequential: Sequential,

    #[command(flatten)]
    pub ingestion: Ingestion,

    #[arg(long, env = "SUI_NETWORK", default_value = "testnet")]
    pub network: Option<AppEnv>,

    #[arg(long, env = "PACKAGES", value_enum, default_values = ["app", "world"], value_delimiter = ',')]
    pub packages: Vec<Package>,

    #[arg(long, env = "METRICS_ADDRESS", default_value = "0.0.0.0:9184")]
    pub metrics_address: SocketAddr,

    #[command(flatten)]
    pub sandbox: SandboxArgs,
}
