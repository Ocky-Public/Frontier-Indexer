use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;

use crate::AppEnv;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Package {
    /// Index your own application data.
    App,

    /// Index the frontier world data.
    World,
}

#[derive(Parser)]
pub struct DbConfig {
    #[arg(long = "db_user", env = "DB_USER", default_value = "postgres")]
    pub db_user: String,

    #[arg(long = "db_password", env = "DB_PASSWORD", default_value = "postgres")]
    pub db_password: String,

    #[arg(long = "db_host", env = "DB_HOST", default_value = "localhost")]
    pub db_host: String,

    #[arg(long = "db_port", env = "DB_PORT", default_value_t = 5432)]
    pub db_port: u16,

    #[arg(long = "db_name", env = "DB_NAME", default_value = "postgres")]
    pub db_name: String,

    #[arg(long = "db_schema", env = "DB_SCHEMA", default_value = "indexer")]
    pub db_schema: String,

    #[arg(
        long = "db_connection_pool_size",
        env = "DB_CONNECTION_POOL_SIZE",
        default_value_t = 100
    )]
    pub db_connection_pool_size: u32,

    #[arg(
        long = "db_connection_timeout_ms",
        env = "DB_CONNECTION_TIMEOUT_MS",
        default_value_t = 60_000
    )]
    pub db_connection_timeout_ms: u64,

    #[arg(long = "db_statement_timeout_ms", env = "DB_STATEMENT_TIMEOUT_MS")]
    pub db_statement_timeout_ms: Option<u64>,

    #[arg(
        long = "tls_verify_cert",
        env = "DB_TLS_VERIFY_CERT",
        default_value_t = false
    )]
    pub tls_verify_cert: bool,

    #[arg(long = "tls_ca_cert_path", env = "DB_TLS_CA_CERT_PATH")]
    pub tls_ca_cert_path: Option<PathBuf>,
}

#[derive(Parser)]
pub struct IndexerConfig {
    #[arg(long = "first_checkpoint", env = "FIRST_CHECKPOINT")]
    pub first_checkpoint: Option<u64>,

    #[arg(long = "last_checkpoint", env = "LAST_CHECKPOINT")]
    pub last_checkpoint: Option<u64>,

    #[arg(long = "pipeline", env = "PIPELINES", value_delimiter = ',')]
    pub pipeline: Vec<String>,
}

#[derive(Parser)]
pub struct Sequential {
    #[arg(long = "checkpoint_lag", env = "CHECKPOINT_LAG", default_value_t = 0)]
    pub checkpoint_lag: u64,

    #[arg(long = "min_eager_rows", env = "MIN_EAGER_ROWS")]
    pub min_eager_rows: Option<usize>,

    #[arg(long = "max_batch_checkpoints", env = "MAX_BATCH_CHECKPOINTS")]
    pub max_batch_checkpoints: Option<usize>,

    #[arg(long = "processor_channel_size", env = "PROCESSOR_CHANNEL_SIZE")]
    pub processor_channel_size: Option<usize>,

    #[arg(
        long = "write_concurrency",
        env = "WRITE_CONCURRENCY",
        default_value_t = 5
    )]
    pub write_concurrency: usize,

    #[arg(
        long = "collect_interval_ms",
        env = "COLLECT_INTERVAL_MS",
        default_value_t = 500
    )]
    pub collect_interval_ms: u64,

    #[arg(
        long = "watermark_interval_ms",
        env = "WATERMARK_INTERVAL_MS",
        default_value_t = 500
    )]
    pub watermark_interval_ms: u64,

    #[arg(
        long = "watermark_interval_jitter_ms",
        env = "WATERMARK_INTERVAL_JITTER_MS",
        default_value_t = 0
    )]
    pub watermark_interval_jitter_ms: u64,
}

#[derive(Parser)]
pub struct Ingestion {
    #[arg(
        long = "checkpoint_buffer_size",
        env = "CHECKPOINT_BUFFER_SIZE",
        default_value_t = 50
    )]
    pub checkpoint_buffer_size: usize,

    #[arg(
        long = "retry_interval_ms",
        env = "RETRY_INTERVAL_MS",
        default_value_t = 200
    )]
    pub retry_interval_ms: u64,

    #[arg(
        long = "streaming_backoff_initial_batch_size",
        env = "STREAMING_BACKOFF_INITIAL_BATCH_SIZE",
        default_value_t = 10
    )]
    pub streaming_backoff_initial_batch_size: usize,

    #[arg(
        long = "streaming_backoff_max_match_size",
        env = "STREAMING_BACKOFF_MAX_BATCH_SIZE",
        default_value_t = 10000
    )]
    pub streaming_backoff_max_batch_size: usize,

    #[arg(
        long = "streaming_connection_timeout_ms",
        env = "STREAMING_CONNECTION_TIMEOUT_MS",
        default_value_t = 5000
    )]
    pub streaming_connection_timeout_ms: u64,

    #[arg(
        long = "streaming_statement_timeout_ms",
        env = "STREAMING_STATEMENT_TIMEOUT_MS",
        default_value_t = 5000
    )]
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
        long = "sandbox_enabled",
        env = "SANDBOX_ENABLED",
        requires = "app_package_ids",
        default_value_t = false
    )]
    pub enabled: bool,

    #[arg(
        long = "sandbox_network",
        env = "SANDBOX_NETWORK",
        default_value = "localnet"
    )]
    pub env: SandboxEnv,

    #[arg(
        long = "sandbox_app_packages",
        env = "SANDBOX_APP_PACKAGES",
        value_delimiter = ','
    )]
    pub app_package_ids: Vec<String>,

    #[clap(
        long = "sandbox_world_packages",
        env = "SANDBOX_WORLD_PACKAGES",
        value_delimiter = ','
    )]
    pub world_packages: Vec<String>,

    #[clap(long = "local_ingestion_path", env = "SANDBOX_INGESTION_PATH")]
    pub local_ingestion_path: Option<PathBuf>,
}

#[derive(Parser)]
pub struct AmqpConfig {
    #[arg(long = "amqp_url", env = "AMQP_URL")]
    pub url: Option<String>,

    #[arg(
        long = "amqp_exchange",
        env = "AMQP_EXCHANGE",
        default_value = "indexer"
    )]
    pub exchange: String,

    #[arg(long = "amqp_pool_size", env = "AMQP_POOL_SIZE", default_value_t = 10)]
    pub pool_size: usize,
}

#[derive(Parser)]
pub struct NatsConfig {
    #[arg(long = "nats_url", env = "NATS_URL")]
    pub url: Option<String>,

    #[arg(
        long = "nats_subject_prefix",
        env = "NATS_SUBJECT_PREFIX",
        default_value = "indexer"
    )]
    pub subject_prefix: String,
}

#[derive(Parser)]
pub struct RedisConfig {
    #[arg(long = "redis_url", env = "REDIS_URL")]
    pub url: Option<String>,

    #[arg(
        long = "redis_channel_prefix",
        env = "REDIS_CHANNEL_PREFIX",
        default_value = "indexer"
    )]
    pub channel_prefix: String,
}

#[derive(Parser)]
pub struct SocketIoConfig {
    #[arg(long = "socket_io_url", env = "SOCKET_IO_URL")]
    pub url: Option<SocketAddr>,
}

#[derive(Parser)]
pub struct TransportConfig {
    #[command(flatten)]
    pub amqp: AmqpConfig,

    #[command(flatten)]
    pub nats: NatsConfig,

    #[command(flatten)]
    pub redis: RedisConfig,

    #[command(flatten)]
    pub socketio: SocketIoConfig,
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

    #[arg(long = "sui_network", env = "SUI_NETWORK", default_value = "testnet")]
    pub network: Option<AppEnv>,

    #[arg(long = "packages", env = "PACKAGES", value_enum, default_values = ["app", "world"], value_delimiter = ',')]
    pub packages: Vec<Package>,

    #[arg(long = "metrics_address", env = "METRICS_ADDRESS", default_value = "0.0.0.0:9184")]
    pub metrics_address: SocketAddr,

    #[command(flatten)]
    pub transport_config: TransportConfig,

    #[command(flatten)]
    pub sandbox: SandboxArgs,
}
