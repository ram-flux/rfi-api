//
//  Copyright 2024 Ram Flux, LLC.
//

mod app;
pub use app::{App, ArcLockAppState, ArcRwLock};

mod errors;
pub use errors::Error;

pub mod fun;
mod logic;

pub mod cache;

pub use logic::account::Account;
pub use logic::account_device::AccountDevice;

pub async fn init_db(uri: &str) -> sqlx::PgPool {
    sqlx::PgPool::connect(uri)
        .await
        .expect("pg pool start failed to connect.")
}

// use anyhow::anyhow;
// use anyhow::Result;
use fred::prelude::*;
use std::time::Duration;
// use tracing::{error, info};

pub async fn init_redis(uri: &str) -> anyhow::Result<RedisClient> {
    tracing::info!("Initializing Redis with URI: {}", uri);

    let config = RedisConfig::from_url(uri)
        .map_err(|e| anyhow::anyhow!("Failed to parse Redis URI: {}", e))?;

    let perf_config = PerformanceConfig {
        auto_pipeline: false,
        max_feed_count: 10,
        ..Default::default()
    };

    let conn_config = ConnectionConfig {
        connection_timeout: Duration::from_secs(1),
        ..Default::default()
    };

    let reconnect_policy = ReconnectPolicy::new_exponential(0, 5000, 2, 0);

    // Use the new constructor to create a RedisClient with configurations
    let client = RedisClient::new(
        config,
        Some(perf_config),
        Some(conn_config),
        Some(reconnect_policy),
    );

    client.init().await?;

    // Perform a ping operation to verify connectivity
    match client.ping::<String>().await {
        Ok(pong) => tracing::info!("PONG IS OK: {}", pong),
        Err(e) => {
            tracing::error!("Failed to ping Redis: {}", e);
            return Err(anyhow::anyhow!("Failed to ping Redis: {}", e));
        }
    }

    tracing::info!("Redis connection established successfully.");
    Ok(client)
}

pub async fn init_redis_pool(uri: &str, max_size: u32) -> anyhow::Result<RedisPool> {
    tracing::info!("Initializing Redis with URI: {}", uri);

    let config = RedisConfig::from_url(uri)
        .map_err(|e| anyhow::anyhow!("Failed to parse Redis URI: {}", e))?;

    let perf_config = PerformanceConfig {
        auto_pipeline: false,
        max_feed_count: 10,
        ..Default::default()
    };

    let conn_config = ConnectionConfig {
        connection_timeout: Duration::from_secs(1),
        ..Default::default()
    };

    let reconnect_policy = ReconnectPolicy::new_exponential(0, 5000, 2, 0);

    let pool = RedisPool::new(
        config,
        Some(perf_config),
        Some(conn_config),
        Some(reconnect_policy),
        max_size as usize,
    )
    .map_err(|e| anyhow::anyhow!("Failed to create Redis pool: {}", e))?;

    pool.init()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize Redis pool: {}", e))?;

    // 设置操作超时时间，以避免可能的挂起
    let _pong: String = tokio::time::timeout(Duration::from_secs(10), pool.ping::<String>())
        .await
        .map_err(|_| anyhow::anyhow!("Timeout when pinging Redis"))?
        .map_err(|e| anyhow::anyhow!("Failed to ping Redis: {}", e))?;

    // match pool.ping::<String>().await {
    //     Ok(pong) => tracing::info!("PONG IS OK: {}", pong),
    //     Err(e) => tracing::error!("Failed to ping Redis: {}", e),
    // }

    tracing::info!("Redis PONG response received successfully.");

    Ok(pool)
}
