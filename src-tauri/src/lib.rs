use db::initialize_db;
use migrations::get_migrations;
use std::sync::Mutex;
use tauri::Manager;

mod command;
mod db;
mod migrations;
mod state;
mod store;

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
            app.manage(state::ClientState::new()?);
            app.manage(Mutex::new(state::CourseState::default()));
            app.manage(Mutex::new(state::LectureState::default()));
            app.manage(Mutex::new(state::PageState::default()));
            let _ = tauri_plugin_store::StoreBuilder::new(app, "store.json")
                .default(store::Settings::KEY, store::Settings::default(app.handle()))
                .build()?;
            let db = initialize_db(&app.handle(), "db.sqlite", &get_migrations())?;
            app.manage(Mutex::new(state::ConnectionState(db)));
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
            command::get_settings::get_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
