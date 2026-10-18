import { useState } from "react"
import "./App.css"

function App() {
  const [count, setCount] = useState(0)

  return (
    <main className="container">
      <h1>Claude Session Manager</h1>
      <p>Cross-platform session and git workflow manager</p>

      <div className="card">
        <p>Coming soon: Session Dashboard, Git Integration, Project Management</p>
      </div>
    </main>
  )
}

export default App
