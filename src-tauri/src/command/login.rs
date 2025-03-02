use crate::state::ClientState;
use collect::iniad::{login_google, login_moocs, Credentials};
use tauri::State;

#[tauri::command]
pub async fn login(
    username: String,
    password: String,
    state: State<'_, ClientState>,
) -> Result<bool, ()> {
    let credentials = Credentials { username, password };
    let logged_in = {
        let client = &state.0;
        let mut logged_in = login_moocs(client, &credentials).await.map_err(|_| ())?;
        logged_in |= login_google(client, &credentials).await.map_err(|_| ())?;
        logged_in
    };
    Ok(logged_in)
}
