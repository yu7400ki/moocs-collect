use tauri::{AppHandle, State};
use thiserror::Error;

use crate::db;
use crate::search::{SearchError, SearchService};
use crate::state::{DbState, SearchState};

#[derive(Debug, Error)]
pub enum PurgeError {
    #[error("Database error: {0}")]
    Database(#[from] db::DbError),
    #[error("Search error: {0}")]
    Search(#[from] SearchError),
}

impl serde::Serialize for PurgeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
pub async fn purge_index(
    app: AppHandle,
    search_state: State<'_, SearchState>,
    db_state: State<'_, DbState>,
) -> Result<(), PurgeError> {
    {
        let mut db_pool = db_state.0.write().await;
        db_pool.close().await;
        *db_pool = db::purge_database(&app).await?;
    }

    {
        let mut search_service = search_state.0.write().await;
        search_service.purge_index()?;
        *search_service = SearchService::try_from(&app)?;
    }

    Ok(())
}
