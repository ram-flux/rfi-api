//
//  Copyright 2024 Ram Flux, LLC.
//

pub fn initialize(log_write: bool, level: &str, dir_name: &str) -> anyhow::Result<()> {
    if log_write {
        _init_log(level, dir_name)
    } else {
        let level = match level {
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => return Err(anyhow::anyhow!("unknown tracing level")),
        };
        tracing_subscriber::fmt()
            .pretty()
            .with_max_level(level)
            .with_writer(std::io::stdout)
            .init();
    }

    tracing::info!("started successfully");
    Ok(())
}

fn _init_log(level: &str, dir_name: &str) {
    use tracing_subscriber::{
        fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
        Registry,
    };
    let dir_name = format!("./{}", dir_name);
    let file_appender = tracing_appender::rolling::daily(dir_name, "debug.log");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
    let formatting_layer = fmt::layer()
        .pretty()
        .with_writer(std::io::stderr)
        .with_writer(file_appender)
        .with_ansi(false);
    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(tracing_error::ErrorLayer::default())
        // .with(fmt::layer())
        .with(formatting_layer)
        .init();
}
