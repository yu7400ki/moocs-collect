use crate::state::CollectState;
use collect::{error::CollectError, Credentials};
use keyring::Entry;
use tauri::State;

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Core library error: {0}")]
    Core(#[from] CollectError),
    #[error("Keyring error: {0}")]
    Keyring(String),
}

impl serde::Serialize for LoginError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
pub async fn login(
    username: String,
    password: String,
    remember: bool,
    state: State<'_, CollectState>,
) -> Result<bool, LoginError> {
    let credentials = Credentials { username, password };
    let collect = &state.collect;

    let authentication_result = collect.authenticate(&credentials).await;
    let logged_in = authentication_result.is_ok();

    if let Err(auth_error) = authentication_result {
        if remember {
            // Even if authentication failed, we might want to clear any stored credentials
            if let Ok(entry) = Entry::new("me.yu7400ki.moocs-collect", &credentials.username) {
                let _ = entry.delete_credential(); // Ignore error if credential doesn't exist
            }
        }
        return Err(LoginError::Core(auth_error));
    }

    if logged_in && remember {
        let entry = Entry::new("me.yu7400ki.moocs-collect", &credentials.username)
            .map_err(|e| LoginError::Keyring(format!("Failed to create keyring entry: {}", e)))?;
        entry
            .set_password(&credentials.password)
            .map_err(|e| LoginError::Keyring(format!("Failed to store password: {}", e)))?;
    }

    Ok(logged_in)
}
