//
//  Copyright 2024 Ram Flux, LLC.
//

mod handler;

//cargo run -p friend
#[tokio::main]
async fn main() {
    let args: common::config::Args = <common::config::Args as clap::Parser>::parse();
    let path = args.config.unwrap_or("./config.toml".to_string());
    let config = common::config::Config::init(&path);

    common::log_init::initialize(config.log_write, &config.log_level, &config.log_dir_name)
        .unwrap();

    let db_conn = models::init_db(config.db_uri.as_str()).await;

    let redis = models::init_redis_pool(&config.redis_uri.as_str(), config.redis_pool_size).await;

    tracing::info!("PONG IS OK:{:#?}", redis);
    let cache_conn = redis.unwrap();

    let app_instance = models::App::new(db_conn, cache_conn);
    let app_instance = tokio::sync::RwLock::new(app_instance);
    let apps = models::ArcRwLock::new(app_instance);
    let state = models::ArcLockAppState(apps);

    let app = axum::Router::new()
        .route("/v1/friend/apply", axum::routing::post(handler::init))
        .fallback(common::fun::handler_404)
        .layer(axum::Extension(state));

    let localhost = format!("0.0.0.0:{}", config.http.friend_port);
    tracing::info!("[localhost:port] {}", localhost);
    let listener = tokio::net::TcpListener::bind(localhost.as_str())
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
