import React, { useState, useEffect } from "react";
import { useGit, Branch } from "../hooks/useGit";
import "./GitTools.css";

export const GitTools: React.FC = () => {
  const { error, loading, getBranches, createBranch, checkoutBranch, createWorktree, listWorktrees } = useGit();
  const [repoPath, setRepoPath] = useState("");
  const [branches, setBranches] = useState<Branch[]>([]);
  const [newBranchName, setNewBranchName] = useState("");
  const [worktreePath, setWorktreePath] = useState("");
  const [showCreateBranch, setShowCreateBranch] = useState(false);
  const [showCreateWorktree, setShowCreateWorktree] = useState(false);

  const handleLoadRepository = async () => {
    if (!repoPath.trim()) {
      alert("Please enter a repository path");
      return;
    }

    try {
      const branchList = await getBranches(repoPath);
      setBranches(branchList);
    } catch (err) {
      console.error("Failed to load repository:", err);
    }
  };

  const handleCreateBranch = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newBranchName.trim()) {
      alert("Please enter a branch name");
      return;
    }

    try {
      await createBranch(repoPath, newBranchName);
      setNewBranchName("");
      setShowCreateBranch(false);
      // Reload branches
      const branchList = await getBranches(repoPath);
      setBranches(branchList);
    } catch (err) {
      console.error("Failed to create branch:", err);
    }
  };

  const handleCheckoutBranch = async (branchName: string) => {
    try {
      await checkoutBranch(repoPath, branchName);
      // Reload branches
      const branchList = await getBranches(repoPath);
      setBranches(branchList);
    } catch (err) {
      console.error("Failed to checkout branch:", err);
    }
  };

  const handleCreateWorktree = async (e: React.FormEvent) => {
    e.preventDefault();
    const selectedBranch = branches.find(b => b.is_current);
    if (!selectedBranch || !worktreePath.trim()) {
      alert("Please select a branch and enter a path");
      return;
    }

    try {
      await createWorktree(repoPath, selectedBranch.name, worktreePath);
      setWorktreePath("");
      setShowCreateWorktree(false);
      alert("Worktree created successfully!");
    } catch (err) {
      console.error("Failed to create worktree:", err);
    }
  };

  return (
    <div className="git-tools">
      <div className="git-section">
        <h3>Repository</h3>
        <div className="git-input-group">
          <input
            type="text"
            placeholder="Enter repository path..."
            value={repoPath}
            onChange={(e) => setRepoPath(e.target.value)}
            className="git-input"
          />
          <button
            onClick={handleLoadRepository}
            disabled={loading}
            className="btn btn-primary"
          >
            {loading ? "Loading..." : "Load Repository"}
          </button>
        </div>
      </div>

      {branches.length > 0 && (
        <>
          <div className="git-section">
            <div className="section-header">
              <h3>Branches</h3>
              <button
                onClick={() => setShowCreateBranch(!showCreateBranch)}
                className="btn btn-secondary"
              >
                + New Branch
              </button>
            </div>

            {showCreateBranch && (
              <form onSubmit={handleCreateBranch} className="git-form">
                <input
                  type="text"
                  placeholder="Branch name..."
                  value={newBranchName}
                  onChange={(e) => setNewBranchName(e.target.value)}
                  className="git-input"
                  autoFocus
                />
                <div className="form-actions">
                  <button type="submit" disabled={loading} className="btn btn-primary">
                    Create
                  </button>
                  <button
                    type="button"
                    onClick={() => setShowCreateBranch(false)}
                    className="btn btn-secondary"
                  >
                    Cancel
                  </button>
                </div>
              </form>
            )}

            <div className="branches-list">
              {branches.map((branch) => (
                <div
                  key={branch.name}
                  className={`branch-item ${branch.is_current ? "current" : ""}`}
                >
                  <div className="branch-info">
                    {branch.is_current && <span className="badge">Current</span>}
                    <span className="branch-name">{branch.name}</span>
                  </div>
                  {!branch.is_current && (
                    <button
                      onClick={() => handleCheckoutBranch(branch.name)}
                      disabled={loading}
                      className="btn btn-secondary"
                    >
                      Checkout
                    </button>
                  )}
                </div>
              ))}
            </div>
          </div>

          <div className="git-section">
            <div className="section-header">
              <h3>Worktrees</h3>
              <button
                onClick={() => setShowCreateWorktree(!showCreateWorktree)}
                className="btn btn-secondary"
              >
                + New Worktree
              </button>
            </div>

            {showCreateWorktree && (
              <form onSubmit={handleCreateWorktree} className="git-form">
                <input
                  type="text"
                  placeholder="Worktree path..."
                  value={worktreePath}
                  onChange={(e) => setWorktreePath(e.target.value)}
                  className="git-input"
                  autoFocus
                />
                <div className="form-actions">
                  <button type="submit" disabled={loading} className="btn btn-primary">
                    Create
                  </button>
                  <button
                    type="button"
                    onClick={() => setShowCreateWorktree(false)}
                    className="btn btn-secondary"
                  >
                    Cancel
                  </button>
                </div>
              </form>
            )}
          </div>
        </>
      )}

      {error && (
        <div className="error-banner">
          <p>{error}</p>
        </div>
      )}
    </div>
  );
};
