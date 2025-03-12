use crate::state::ClientState;
use collect::iniad::{login_google, login_moocs, Credentials};
use keyring::Entry;
use tauri::State;

#[tauri::command]
pub async fn login(
    username: String,
    password: String,
    remember: bool,
    state: State<'_, ClientState>,
) -> Result<bool, ()> {
    let credentials = Credentials { username, password };
    let logged_in = {
        let client = &state.0;
        let mut logged_in = login_moocs(client, &credentials).await.map_err(|_| ())?;
        logged_in |= login_google(client, &credentials).await.map_err(|_| ())?;
        logged_in
    };
    if logged_in && remember {
        let entry =
            Entry::new("me.yu7400ki.moocs-collect", &credentials.username).map_err(|_| ())?;
        entry.set_password(&credentials.password).map_err(|_| ())?;
    }
    Ok(logged_in)
}
