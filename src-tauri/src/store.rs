use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

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

    pub fn get(app: &AppHandle) -> Self {
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
