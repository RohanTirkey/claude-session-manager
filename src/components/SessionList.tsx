import React, { useState } from "react";
import { Session } from "../hooks/useSessions";
import "./SessionList.css";

interface SessionListProps {
  sessions: Session[];
  loading: boolean;
  onDelete: (id: string) => Promise<void>;
  onUpdate: (id: string, name: string, tags?: string[]) => Promise<void>;
}

export const SessionList: React.FC<SessionListProps> = ({
  sessions,
  loading,
  onDelete,
  onUpdate,
}) => {
  const [filter, setFilter] = useState("");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [editName, setEditName] = useState("");

  if (loading) {
    return <div className="session-list loading">Loading sessions...</div>;
  }

  if (sessions.length === 0) {
    return (
      <div className="session-list empty">
        <p>No sessions yet. Create one to get started!</p>
      </div>
    );
  }

  const filteredSessions = sessions.filter(
    (session) =>
      session.name.toLowerCase().includes(filter.toLowerCase()) ||
      (session.project?.toLowerCase().includes(filter.toLowerCase()) ?? false)
  );

  const handleEdit = (session: Session) => {
    setSelectedId(session.id);
    setEditName(session.name);
  };

  const handleSave = async (id: string) => {
    if (editName.trim()) {
      try {
        await onUpdate(id, editName);
        setSelectedId(null);
      } catch (err) {
        console.error("Failed to update session:", err);
      }
    }
  };

  const handleDelete = async (id: string) => {
    if (window.confirm("Are you sure you want to delete this session?")) {
      try {
        await onDelete(id);
      } catch (err) {
        console.error("Failed to delete session:", err);
      }
    }
  };

  return (
    <div className="session-list">
      <div className="session-list-header">
        <h2>Sessions</h2>
        <input
          type="text"
          placeholder="Search sessions..."
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          className="session-filter"
        />
      </div>

      <div className="session-items">
        {filteredSessions.map((session) => (
          <div
            key={session.id}
            className={`session-item ${selectedId === session.id ? "editing" : ""}`}
          >
            <div className="session-content">
              {selectedId === session.id ? (
                <input
                  type="text"
                  value={editName}
                  onChange={(e) => setEditName(e.target.value)}
                  className="session-name-input"
                  autoFocus
                />
              ) : (
                <h3 className="session-name">{session.name}</h3>
              )}

              {session.project && (
                <p className="session-project">{session.project}</p>
              )}

              <div className="session-meta">
                <span className="session-date">
                  {new Date(session.created_at).toLocaleDateString()}
                </span>
                {session.tags && session.tags.length > 0 && (
                  <div className="session-tags">
                    {session.tags.map((tag) => (
                      <span key={tag} className="tag">
                        {tag}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            </div>

            <div className="session-actions">
              {selectedId === session.id ? (
                <>
                  <button
                    className="btn btn-primary"
                    onClick={() => handleSave(session.id)}
                  >
                    Save
                  </button>
                  <button
                    className="btn btn-secondary"
                    onClick={() => setSelectedId(null)}
                  >
                    Cancel
                  </button>
                </>
              ) : (
                <>
                  <button
                    className="btn btn-secondary"
                    onClick={() => handleEdit(session)}
                  >
                    Edit
                  </button>
                  <button
                    className="btn btn-danger"
                    onClick={() => handleDelete(session.id)}
                  >
                    Delete
                  </button>
                </>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
