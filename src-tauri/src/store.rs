use std::sync::Mutex;

use chrono::prelude::*;
use rusqlite::Transaction;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use crate::state::ConnectionState;

const STORE_NAME: &str = "store.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub theme: Theme,
    pub download_dir: String,
    pub year: Option<u32>,
}

impl Settings {
    pub const KEY: &'static str = "settings";

    pub fn default(app: &AppHandle) -> Self {
        let theme = Theme::System;
        let document_dir = app.path().document_dir().unwrap_or_default();
        let download_dir = document_dir
            .join("moocs-collect")
            .to_string_lossy()
            .to_string();
        let year = None;
        Self {
            theme,
            download_dir,
            year,
        }
    }
}

impl From<&AppHandle> for Settings {
    fn from(app: &AppHandle) -> Self {
        app.get_store(STORE_NAME)
            .expect("failed to get store")
            .get(Self::KEY)
            .map(|value| value.try_into().unwrap_or_else(|_| Self::default(app)))
            .unwrap_or_else(|| Self::default(app))
    }
}

impl Into<Value> for Settings {
    fn into(self) -> Value {
        serde_json::to_value(self).expect("failed to convert Settings into Value")
    }
}

impl TryFrom<Value> for Settings {
    type Error = serde_json::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageCacheEntry {
    pub url: String,
    pub path: String,
    pub last_modified: DateTime<Utc>,
}

impl ImageCacheEntry {
    pub fn new(url: impl ToString, path: impl ToString) -> Self {
        Self {
            url: url.to_string(),
            path: path.to_string(),
            last_modified: Utc::now(),
        }
    }
}

pub struct ImageCache<'a> {
    db: &'a Mutex<ConnectionState>,
}

impl<'a> ImageCache<'a> {
    pub fn new(app: &'a AppHandle) -> Self {
        let db = app.state::<Mutex<ConnectionState>>().inner();
        Self { db }
    }

    fn tx<T>(
        &self,
        f: impl FnOnce(&Transaction) -> Result<T, rusqlite::Error>,
    ) -> Result<T, rusqlite::Error> {
        let mut guard = self.db.lock().expect("failed to lock db");
        let db = &mut *guard;
        let tx = db.0.transaction()?;
        let result = f(&tx);
        if result.is_ok() {
            tx.commit()?;
        } else {
            tx.rollback()?;
        }
        result
    }

    pub fn insert(&self, entry: ImageCacheEntry) -> Result<(), rusqlite::Error> {
        self.tx(|tx| {
            tx.execute(
                "INSERT INTO image_cache (url, path, last_modified) VALUES (?1, ?2, ?3)
                ON CONFLICT(url) DO UPDATE SET path = ?2, last_modified = ?3",
                (&entry.url, &entry.path, &entry.last_modified.timestamp()),
            )
        })?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<ImageCacheEntry>, rusqlite::Error> {
        self.tx(|tx| {
            let mut stmt =
                tx.prepare("SELECT path, last_modified FROM image_cache WHERE url = ?1")?;
            let mut rows = stmt.query(&[key])?;
            if let Some(row) = rows.next()? {
                let path: String = row.get(0)?;
                let timestamp: i64 = row.get(1)?;
                let last_modified = Utc.timestamp_opt(timestamp, 0).single().unwrap();
                Ok(Some(ImageCacheEntry {
                    url: key.to_string(),
                    path,
                    last_modified,
                }))
            } else {
                Ok(None)
            }
        })
    }
}
