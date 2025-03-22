use rusqlite::Connection;
use std::fs::create_dir_all;
use tauri::{AppHandle, Manager};

#[derive(Debug)]
pub struct Migration {
    pub version: u32,
    pub sql: &'static str,
}

pub fn initialize_db(
    app: &AppHandle,
    conn_url: &str,
    migrations: &[Migration],
) -> Result<Connection, rusqlite::Error> {
    let app_path = app
        .path()
        .app_config_dir()
        .expect("failed to get app config dir");

    create_dir_all(&app_path).expect("failed to create app config dir");

    let conn = app_path.join(conn_url);

    let mut db = Connection::open(&conn)?;

    let user_version = {
        let mut stmt = db.prepare("PRAGMA user_version")?;
        let user_version = stmt.query_row([], |row| row.get::<_, u32>(0))?;
        user_version
    };

    upgrade_db(&mut db, migrations, user_version)?;

    Ok(db)
}

pub fn upgrade_db(
    db: &mut Connection,
    migrations: &[Migration],
    user_version: u32,
) -> Result<(), rusqlite::Error> {
    let tx = db.transaction()?;
    for migration in migrations.iter().filter(|m| m.version > user_version) {
        tx.execute_batch(migration.sql)?;
        tx.pragma_update(None, "user_version", &migration.version)?;
    }
    tx.commit()?;
    Ok(())
}
