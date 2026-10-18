import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";

export interface Session {
  id: string;
  name: string;
  project?: string;
  context?: string;
  created_at: string;
  updated_at: string;
  tags?: string[];
}

export const useSessions = () => {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    initializeAndFetchSessions();
  }, []);

  const initializeAndFetchSessions = async () => {
    try {
      setLoading(true);
      // Initialize database
      await invoke("init_db");
      // Fetch sessions
      const data = await invoke<Session[]>("get_sessions");
      setSessions(data);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  };

  const createSession = async (name: string, project?: string) => {
    try {
      const session = await invoke<Session>("create_session", { name, project });
      setSessions([session, ...sessions]);
      return session;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    }
  };

  const deleteSession = async (id: string) => {
    try {
      await invoke("delete_session", { id });
      setSessions(sessions.filter(s => s.id !== id));
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    }
  };

  const updateSession = async (id: string, name: string, tags?: string[]) => {
    try {
      await invoke("update_session", { id, name, tags });
      setSessions(sessions.map(s =>
        s.id === id ? { ...s, name, tags } : s
      ));
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    }
  };

  const saveSessionContext = async (id: string, context: string) => {
    try {
      await invoke("save_session_context", { id, context });
      setSessions(sessions.map(s =>
        s.id === id ? { ...s, context } : s
      ));
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    }
  };

  const getSessionsByProject = async (project: string) => {
    try {
      const data = await invoke<Session[]>("get_sessions_by_project", { project });
      return data;
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      throw err;
    }
  };

  return {
    sessions,
    loading,
    error,
    createSession,
    deleteSession,
    updateSession,
    saveSessionContext,
    getSessionsByProject,
    refresh: initializeAndFetchSessions,
  };
};
