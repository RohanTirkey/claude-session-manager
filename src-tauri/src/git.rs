use serde::{Deserialize, Serialize};
use anyhow::Result;
use git2::{Repository, BranchType};
use std::path::Path;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitRepo {
    pub path: String,
    pub name: String,
}

/// Open a git repository and validate it exists
pub fn open_repo(repo_path: &str) -> Result<Repository> {
    let repo = Repository::open(repo_path)?;
    Ok(repo)
}

/// List all branches in a repository
pub fn list_branches(repo_path: &str) -> Result<Vec<Branch>> {
    let repo = open_repo(repo_path)?;
    let mut branches = Vec::new();

    // Get the HEAD reference to determine current branch
    let head = repo.head().ok();
    let current_branch = head.as_ref().and_then(|h| h.shorthand()).unwrap_or("");

    // Iterate through all branches
    let branch_iter = repo.branches(Some(BranchType::Local))?;

    for branch in branch_iter {
        if let Ok((branch, _)) = branch {
            if let Ok(name) = branch.name() {
                if let Some(name) = name {
                    let is_current = name == current_branch;
                    branches.push(Branch {
                        name: name.to_string(),
                        is_current,
                    });
                }
            }
        }
    }

    branches.sort_by(|a, b| {
        // Current branch first
        if a.is_current {
            std::cmp::Ordering::Less
        } else if b.is_current {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    Ok(branches)
}

/// Create a new branch from the current HEAD
pub fn create_branch(repo_path: &str, branch_name: &str) -> Result<Branch> {
    let repo = open_repo(repo_path)?;
    let head = repo.head()?;
    let commit = repo.find_commit(head.target().unwrap().into())?;

    repo.branch(branch_name, &commit, false)?;

    Ok(Branch {
        name: branch_name.to_string(),
        is_current: false,
    })
}

/// Delete a branch
pub fn delete_branch(repo_path: &str, branch_name: &str) -> Result<()> {
    let repo = open_repo(repo_path)?;
    let mut branch = repo.find_branch(branch_name, BranchType::Local)?;
    branch.delete()?;
    Ok(())
}

/// Checkout a branch
pub fn checkout_branch(repo_path: &str, branch_name: &str) -> Result<()> {
    let repo = open_repo(repo_path)?;
    let branch = repo.find_branch(branch_name, BranchType::Local)?;
    let obj = repo.reference_to_annotated_commit(branch.get())?;
    repo.set_head_detached(obj.id())?;

    // Update working directory
    let mut checkout_builder = git2::build::CheckoutBuilder::new();
    checkout_builder.force();
    repo.checkout_tree(&repo.find_object(obj.id(), None)?, Some(&mut checkout_builder))?;

    Ok(())
}

/// Create a new worktree
pub fn create_worktree(repo_path: &str, worktree_path: &str, branch_name: &str) -> Result<Worktree> {
    let repo = open_repo(repo_path)?;

    // Check if branch exists, if not create it
    let branch_exists = repo.find_branch(branch_name, BranchType::Local).is_ok();

    if !branch_exists {
        let head = repo.head()?;
        let commit = repo.find_commit(head.target().unwrap().into())?;
        repo.branch(branch_name, &commit, false)?;
    }

    // Create worktree
    let mut worktree_builder = repo.worktree(branch_name, Path::new(worktree_path), None)?;

    Ok(Worktree {
        path: worktree_path.to_string(),
        branch: branch_name.to_string(),
    })
}

/// Delete a worktree
pub fn delete_worktree(repo_path: &str, worktree_name: &str) -> Result<()> {
    let repo = open_repo(repo_path)?;

    // Find and delete the worktree
    let mut worktree = repo.find_worktree(worktree_name)?;
    worktree.unlock()?;
    repo.remove_worktree(worktree_name)?;

    Ok(())
}

/// List all worktrees in a repository
pub fn list_worktrees(repo_path: &str) -> Result<Vec<Worktree>> {
    let repo = open_repo(repo_path)?;
    let mut worktrees = Vec::new();

    let worktree_iter = repo.worktrees()?;

    for worktree_name in worktree_iter.iter() {
        if let Ok(worktree_name) = worktree_name {
            if let Ok(worktree) = repo.find_worktree(&worktree_name) {
                if let Ok(path) = worktree.path().to_str() {
                    // Try to get the branch name from the worktree
                    let branch_name = worktree_name.to_string();
                    worktrees.push(Worktree {
                        path: path.to_string(),
                        branch: branch_name,
                    });
                }
            }
        }
    }

    Ok(worktrees)
}

/// Detect git repositories in a directory
pub fn discover_repos(start_path: &str) -> Result<Vec<GitRepo>> {
    let mut repos = Vec::new();
    let path = Path::new(start_path);

    if path.is_dir() {
        // Check current directory
        if let Ok(repo) = Repository::discover(path) {
            if let Ok(name) = repo.workdir()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
            {
                repos.push(GitRepo {
                    path: repo.workdir().unwrap().to_string_lossy().to_string(),
                    name: name.to_string(),
                });
            }
        }
    }

    Ok(repos)
}

