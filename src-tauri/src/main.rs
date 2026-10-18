#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod commands;
mod db;
mod git;
mod session;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::get_sessions,
      commands::create_session,
      commands::delete_session,
      commands::update_session,
      commands::get_git_branches,
      commands::create_worktree,
      commands::delete_worktree,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
