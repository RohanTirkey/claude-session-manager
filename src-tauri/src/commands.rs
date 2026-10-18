use crate::session::Session;
use crate::db::Database;
use std::sync::Mutex;
use tauri::State;
use anyhow::Result;

// Global database state
pub struct DbState {
    pub db: Mutex<Option<Database>>,
}

#[tauri::command]
pub async fn init_db(state: State<'_, DbState>) -> Result<(), String> {
    let app_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("claude-session-manager");

    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;

    let db_path = app_dir.join("sessions.db");
    let db = Database::new(db_path).map_err(|e| e.to_string())?;

    *state.db.lock().unwrap() = Some(db);
    Ok(())
}

#[tauri::command]
pub async fn get_sessions(state: State<'_, DbState>) -> Result<Vec<Session>, String> {
    let db = state.db.lock().unwrap();
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.get_all_sessions().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_session(
    name: String,
    project: Option<String>,
    state: State<'_, DbState>,
) -> Result<Session, String> {
    let db = state.db.lock().unwrap();
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.create_session(name, project).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_session(id: String, state: State<'_, DbState>) -> Result<Option<Session>, String> {
    let db = state.db.lock().unwrap();
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.get_session(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_session(id: String, state: State<'_, DbState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.delete_session(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_session(
    id: String,
    name: String,
    tags: Option<Vec<String>>,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.update_session(id, name, tags)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_session_context(
    id: String,
    context: String,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.save_session_context(&id, context)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_sessions_by_project(
    project: String,
    state: State<'_, DbState>,
) -> Result<Vec<Session>, String> {
    let db = state.db.lock().unwrap();
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.get_sessions_by_project(&project)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_git_branches(repo_path: String) -> Result<Vec<String>, String> {
    // TODO: Implement using git2-rs wrapper
    Ok(vec![])
}

#[tauri::command]
pub async fn create_worktree(
    repo_path: String,
    branch: String,
    path: String,
) -> Result<(), String> {
    // TODO: Implement using git2-rs wrapper
    Ok(())
}

#[tauri::command]
pub async fn delete_worktree(path: String) -> Result<(), String> {
    // TODO: Implement using git2-rs wrapper
    Ok(())
}
