use crate::session::Session;
use serde_json::json;

#[tauri::command]
pub async fn get_sessions() -> Result<Vec<Session>, String> {
    // TODO: Fetch from database
    Ok(vec![])
}

#[tauri::command]
pub async fn create_session(name: String, project: Option<String>) -> Result<Session, String> {
    // TODO: Create session in database
    Ok(Session {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        project,
        context: None,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        tags: None,
    })
}

#[tauri::command]
pub async fn delete_session(id: String) -> Result<(), String> {
    // TODO: Delete from database
    Ok(())
}

#[tauri::command]
pub async fn update_session(id: String, name: String, tags: Option<Vec<String>>) -> Result<(), String> {
    // TODO: Update in database
    Ok(())
}

#[tauri::command]
pub async fn get_git_branches(repo_path: String) -> Result<Vec<String>, String> {
    // TODO: Get branches using git module
    Ok(vec![])
}

#[tauri::command]
pub async fn create_worktree(repo_path: String, branch: String, path: String) -> Result<(), String> {
    // TODO: Create worktree using git module
    Ok(())
}

#[tauri::command]
pub async fn delete_worktree(path: String) -> Result<(), String> {
    // TODO: Delete worktree using git module
    Ok(())
}
