use std::collections::HashSet;

use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqlitePool};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug)]
pub struct Migration {
    pub statements: &'static [&'static str],
    pub folder_millis: i64,
    pub hash: &'static str,
    pub tag: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/migrations.rs"));

pub async fn run_pending_migrations(pool: &SqlitePool) -> Result<(), MigrationError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS __drizzle_migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            hash TEXT NOT NULL,
            created_at INTEGER
        )",
    )
    .execute(pool)
    .await?;

    let mut applied_hashes = sqlx::query("SELECT hash FROM __drizzle_migrations")
        .map(|row: SqliteRow| row.get::<String, _>("hash"))
        .fetch_all(pool)
        .await?
        .into_iter()
        .collect::<HashSet<String>>();

    for migration in MIGRATIONS.iter() {
        if applied_hashes.contains(migration.hash) {
            continue;
        }

        log::info!("Applying migration {}", migration.tag);
        apply_migration(pool, migration).await?;
        applied_hashes.insert(migration.hash.to_owned());
    }

    Ok(())
}

async fn apply_migration(pool: &SqlitePool, migration: &Migration) -> Result<(), MigrationError> {
    let mut tx = pool.begin().await?;

    for statement in migration.statements {
        sqlx::query(statement).execute(&mut *tx).await?;
    }

    sqlx::query("INSERT INTO __drizzle_migrations (hash, created_at) VALUES (?, ?)")
        .bind(migration.hash)
        .bind(migration.folder_millis)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}
