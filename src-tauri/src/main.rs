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
      // Session commands
      commands::init_db,
      commands::get_sessions,
      commands::create_session,
      commands::get_session,
      commands::delete_session,
      commands::update_session,
      commands::save_session_context,
      commands::get_sessions_by_project,
      // Git commands
      commands::get_git_branches,
      commands::create_git_branch,
      commands::delete_git_branch,
      commands::checkout_git_branch,
      commands::create_worktree,
      commands::delete_worktree,
      commands::list_worktrees,
      commands::discover_git_repos,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
