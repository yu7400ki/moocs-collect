use tauri::{Builder, Manager};

mod command;
mod state;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.manage(state::ClientState::new()?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, command::login::login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
