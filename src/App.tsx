import { useState, useMemo } from "react"
import "./App.css"
import { useSessions } from "./hooks/useSessions"
import { SessionList } from "./components/SessionList"
import { CreateSession } from "./components/CreateSession"
import { GitTools } from "./components/GitTools"

function App() {
  const { sessions, loading, error, createSession, deleteSession, updateSession } = useSessions()
  const [activeTab, setActiveTab] = useState<"sessions" | "git">("sessions")

  const projects = useMemo(() => {
    const projectSet = new Set<string>()
    sessions.forEach(s => {
      if (s.project) projectSet.add(s.project)
    })
    return Array.from(projectSet).sort()
  }, [sessions])

  return (
    <main className="app">
      <header className="app-header">
        <h1>Claude Session Manager</h1>
        <p>Manage your Claude sessions and git workflows</p>
      </header>

      {error && (
        <div className="error-banner">
          <p>{error}</p>
        </div>
      )}

      <div className="app-content">
        <nav className="app-nav">
          <button
            className={`nav-button ${activeTab === "sessions" ? "active" : ""}`}
            onClick={() => setActiveTab("sessions")}
          >
            Sessions
          </button>
          <button
            className={`nav-button ${activeTab === "git" ? "active" : ""}`}
            onClick={() => setActiveTab("git")}
          >
            Git Tools
          </button>
        </nav>

        <div className="app-main">
          {activeTab === "sessions" && (
            <div className="tab-content">
              <CreateSession
                onCreate={createSession}
                projects={projects}
              />
              <SessionList
                sessions={sessions}
                loading={loading}
                onDelete={deleteSession}
                onUpdate={updateSession}
              />
            </div>
          )}

          {activeTab === "git" && (
            <div className="tab-content">
              <GitTools />
            </div>
          )}
        </div>
      </div>
    </main>
  )
}

export default App


