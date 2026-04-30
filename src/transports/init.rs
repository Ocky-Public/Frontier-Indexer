use std::sync::Arc;

use anyhow::Context;
use serde::Serialize;
use socketioxide::SocketIo;

use crate::config::TransportConfig;
use crate::transports::AmqpTransport;
use crate::transports::NatsTransport;
use crate::transports::RedisTransport;
use crate::transports::SocketIoTransport;
use crate::transports::Transport;

pub enum TransportOption {
    Amqp(Arc<AmqpTransport>),
    Nats(Arc<NatsTransport>),
    Redis(Arc<RedisTransport>),
    SocketIo(Arc<SocketIoTransport>),
}

pub struct Transports {
    transports: Vec<TransportOption>,
}

impl Transports {
    pub async fn init(cfg: &TransportConfig) -> anyhow::Result<Self> {
        let mut transports = Vec::new();

        // SocketIO
        if let Some(addr) = cfg.socketio.url {
            let (layer, io) = SocketIo::new_layer();
            let app = axum::Router::new().layer(layer);
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .with_context(|| format!("Failed to bind Socket.IO server to {}", addr))?;

            tokio::spawn(async move {
                axum::serve(listener, app)
                    .await
                    .expect("Socket.IO HTTP server failed");
            });

            tracing::info!("Socket.IO transport listening on {}", addr);
            let transport = Arc::new(SocketIoTransport::new("socket.io", io));
            transports.push(TransportOption::SocketIo(transport));
        }

        // AMQP
        if let Some(url) = &cfg.amqp.url {
            let transport =
                AmqpTransport::connect("amqp", url, &cfg.amqp.exchange, cfg.amqp.pool_size)
                    .await
                    .context("Failed to connect AMQP transport")?;

            tracing::info!("AMQP transport connected to {}", url);
            transports.push(TransportOption::Amqp(Arc::new(transport)));
        }

        // NATS
        if let Some(url) = &cfg.nats.url {
            let transport = NatsTransport::connect("nats", url, &cfg.nats.subject_prefix)
                .await
                .context("Failed to connect NATS transport")?;

            tracing::info!("NATS transport connected to {}", url);
            transports.push(TransportOption::Nats(Arc::new(transport)));
        }

        // Redis
        if let Some(url) = &cfg.redis.url {
            let transport = RedisTransport::connect("redis", url, &cfg.redis.channel_prefix)
                .await
                .context("Failed to connect Redis transport")?;

            tracing::info!("Redis transport connected to {}", url);
            transports.push(TransportOption::Redis(Arc::new(transport)));
        }

        if transports.is_empty() {
            tracing::info!("No transports configured - all pipelines will run without emitting.");
        } else {
            tracing::info!("{} transport(s) active.", transports.len());
        }

        Ok(Self { transports })
    }

    pub fn for_pipeline<I>(&self) -> Vec<Arc<dyn Transport<I>>>
    where
        I: Serialize + Send + Sync + 'static,
        AmqpTransport: Transport<I>,
        NatsTransport: Transport<I>,
        RedisTransport: Transport<I>,
        SocketIoTransport: Transport<I>,
    {
        self.transports
            .iter()
            .map(|transport| match transport {
                TransportOption::Amqp(entry) => Arc::clone(entry) as Arc<dyn Transport<I>>,
                TransportOption::Nats(entry) => Arc::clone(entry) as Arc<dyn Transport<I>>,
                TransportOption::Redis(entry) => Arc::clone(entry) as Arc<dyn Transport<I>>,
                TransportOption::SocketIo(entry) => Arc::clone(entry) as Arc<dyn Transport<I>>,
            })
            .collect()
    }
}
