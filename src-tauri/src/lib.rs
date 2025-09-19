use tauri::{async_runtime, Manager};

mod command;
mod db;
mod search;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle();

            if cfg!(debug_assertions) {
                handle.plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let db_pool = async_runtime::block_on(db::init(handle.clone()))?;

            app.manage(db_pool);
            app.manage(state::CollectState::new()?);
            app.manage(state::SearchState::new(app)?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::download_slides,
            command::login,
            command::get_courses,
            command::get_credential,
            command::get_lectures,
            command::get_pages,
            command::get_archive_years,
            command::search_slides,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
