#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod commands;
mod db;
mod git;
mod session;

use commands::DbState;
use std::sync::Mutex;

fn main() {
  tauri::Builder::default()
    .manage(DbState {
      db: Mutex::new(None),
    })
    .setup(|app| {
      let app_handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        if let Err(e) = commands::init_db(tauri::State::new(DbState {
          db: Mutex::new(None),
        })).await {
          eprintln!("Failed to initialize database: {}", e);
        }
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::init_db,
      commands::get_sessions,
      commands::create_session,
      commands::get_session,
      commands::delete_session,
      commands::update_session,
      commands::save_session_context,
      commands::get_sessions_by_project,
      commands::get_git_branches,
      commands::create_worktree,
      commands::delete_worktree,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
