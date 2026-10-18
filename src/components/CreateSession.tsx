import React, { useState } from "react";
import "./CreateSession.css";

interface CreateSessionProps {
  onCreate: (name: string, project?: string) => Promise<void>;
  projects: string[];
}

export const CreateSession: React.FC<CreateSessionProps> = ({
  onCreate,
  projects,
}) => {
  const [name, setName] = useState("");
  const [project, setProject] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) {
      setError("Session name is required");
      return;
    }

    try {
      setLoading(true);
      setError(null);
      await onCreate(name, project || undefined);
      setName("");
      setProject("");
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="create-session">
      <h3>Create New Session</h3>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="session-name">Session Name *</label>
          <input
            id="session-name"
            type="text"
            placeholder="e.g., Feature X Development"
            value={name}
            onChange={(e) => setName(e.target.value)}
            disabled={loading}
            autoFocus
          />
        </div>

        <div className="form-group">
          <label htmlFor="session-project">Project (Optional)</label>
          <select
            id="session-project"
            value={project}
            onChange={(e) => setProject(e.target.value)}
            disabled={loading}
          >
            <option value="">-- Select a project --</option>
            {projects.map((p) => (
              <option key={p} value={p}>
                {p}
              </option>
            ))}
          </select>
        </div>

        {error && <div className="error-message">{error}</div>}

        <button type="submit" disabled={loading} className="btn-submit">
          {loading ? "Creating..." : "Create Session"}
        </button>
      </form>
    </div>
  );
};
