use std::path::PathBuf;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use tauri::AppHandle;
use tauri::Manager;
use thiserror::Error;

mod migrator;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("failed to resolve application data directory: {0}")]
    Path(String),
    #[error("failed to prepare database directory: {0}")]
    Io(#[from] std::io::Error),
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migration(#[from] migrator::MigrationError),
}

pub async fn init(handle: AppHandle) -> Result<SqlitePool, DbError> {
    let db_path = resolve_db_path(&handle)?;
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let connect_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    migrator::run_pending_migrations(&pool).await?;

    Ok(pool)
}

pub async fn purge_database(handle: &AppHandle) -> Result<SqlitePool, DbError> {
    let db_path = resolve_db_path(handle)?;

    if db_path.exists() {
        std::fs::remove_file(&db_path)?;
    }

    let pool = init(handle.clone()).await?;

    Ok(pool)
}

fn resolve_db_path(handle: &AppHandle) -> Result<PathBuf, DbError> {
    let base_dir = handle
        .path()
        .app_data_dir()
        .map_err(|err| DbError::Path(err.to_string()))?;
    Ok(base_dir.join("db.sqlite"))
}
