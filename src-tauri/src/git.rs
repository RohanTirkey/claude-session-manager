use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    pub name: String,
    pub is_current: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Worktree {
    pub path: String,
    pub branch: String,
}

pub async fn list_branches(repo_path: &str) -> Result<Vec<Branch>> {
    // TODO: Implement using simple-git wrapper
    Ok(vec![])
}

pub async fn create_worktree(repo_path: &str, branch_name: &str, path: &str) -> Result<()> {
    // TODO: Implement using simple-git wrapper
    Ok(())
}

pub async fn delete_worktree(path: &str) -> Result<()> {
    // TODO: Implement using simple-git wrapper
    Ok(())
}
