import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

export interface Branch {
  name: string;
  is_current: boolean;
}

export interface Worktree {
  path: string;
  branch: string;
}

export const useGit = () => {
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const getBranches = async (repoPath: string) => {
    try {
      setLoading(true);
      const branches = await invoke<Branch[]>("get_git_branches", {
        repo_path: repoPath,
      });
      setError(null);
      return branches;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const createBranch = async (repoPath: string, branchName: string) => {
    try {
      setLoading(true);
      const branch = await invoke<Branch>("create_git_branch", {
        repo_path: repoPath,
        branch_name: branchName,
      });
      setError(null);
      return branch;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const deleteBranch = async (repoPath: string, branchName: string) => {
    try {
      setLoading(true);
      await invoke("delete_git_branch", {
        repo_path: repoPath,
        branch_name: branchName,
      });
      setError(null);
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const checkoutBranch = async (repoPath: string, branchName: string) => {
    try {
      setLoading(true);
      await invoke("checkout_git_branch", {
        repo_path: repoPath,
        branch_name: branchName,
      });
      setError(null);
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const createWorktree = async (
    repoPath: string,
    branch: string,
    path: string
  ) => {
    try {
      setLoading(true);
      const worktree = await invoke<Worktree>("create_worktree", {
        repo_path: repoPath,
        branch,
        path,
      });
      setError(null);
      return worktree;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const deleteWorktree = async (
    repoPath: string,
    worktreeName: string
  ) => {
    try {
      setLoading(true);
      await invoke("delete_worktree", {
        repo_path: repoPath,
        worktree_name: worktreeName,
      });
      setError(null);
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const listWorktrees = async (repoPath: string) => {
    try {
      setLoading(true);
      const worktrees = await invoke<Worktree[]>("list_worktrees", {
        repo_path: repoPath,
      });
      setError(null);
      return worktrees;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const discoverRepos = async (path: string) => {
    try {
      setLoading(true);
      const repos = await invoke<Array<{ path: string; name: string }>>(
        "discover_git_repos",
        { path }
      );
      setError(null);
      return repos;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  return {
    error,
    loading,
    getBranches,
    createBranch,
    deleteBranch,
    checkoutBranch,
    createWorktree,
    deleteWorktree,
    listWorktrees,
    discoverRepos,
  };
};
