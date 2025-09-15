use tauri::Manager;

mod command;
mod search;
mod state;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.manage(state::CollectState::new()?);
            app.manage(state::SearchState::new(app)?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            command::download_slides::download_slides,
            command::login::login,
            command::get_courses::get_courses,
            command::get_credential::get_credential,
            command::get_lectures::get_lectures,
            command::get_pages::get_pages,
            command::get_archive_years::get_archive_years,
            command::search_slides::search_slides,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
