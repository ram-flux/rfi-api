//
//  Copyright 2024 Ram Flux, LLC.
//


#[derive(Debug, Clone)]
pub struct App {
    pub db: sqlx::PgPool,
    pub rd: fred::clients::RedisPool,
}

impl App {
    pub fn new(db: sqlx::PgPool, rd: fred::clients::RedisPool) -> App {
        App { db, rd }
    }
}


pub type ArcRwLock<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

#[derive(Debug, Clone)]
pub struct ArcLockAppState(pub std::sync::Arc<tokio::sync::RwLock<App>>);

impl std::ops::Deref for ArcLockAppState {
    type Target = std::sync::Arc<tokio::sync::RwLock<App>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArcLockAppState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
