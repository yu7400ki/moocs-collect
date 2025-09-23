use crate::search::{SearchError, SearchService};
use collect::Collect;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CollectState {
    pub collect: Arc<Collect>,
    pub client: Arc<Client>,
}

impl CollectState {
    pub fn new() -> reqwest::Result<Self> {
        let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0")
        .cookie_store(true)
        .build()?;
        let client = Arc::new(client);
        Ok(Self {
            collect: Arc::new(Collect::from(client.clone())),
            client,
        })
    }
}

pub struct SearchState(pub RwLock<SearchService>);

impl SearchState {
    pub fn new(app: &tauri::App) -> Result<Self, SearchError> {
        let handle = app.handle();
        let search_service = SearchService::try_from(handle)?;
        Ok(Self(RwLock::new(search_service)))
    }
}

pub struct DbState(pub RwLock<sqlx::SqlitePool>);

impl DbState {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self(RwLock::new(pool))
    }
}
