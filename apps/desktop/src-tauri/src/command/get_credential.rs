use keyring::Entry;

#[tauri::command]
pub async fn get_credential(username: String) -> Result<Option<String>, ()> {
    if username.is_empty() {
        return Ok(None);
    }
    let entry = Entry::new("me.yu7400ki.moocs-collect", &username).map_err(|_| ())?;
    let password = entry.get_password().ok();
    Ok(password)
}
