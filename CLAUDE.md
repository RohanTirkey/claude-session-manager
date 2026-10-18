# Claude Session Manager - Development Guide

## Project Overview

A cross-platform desktop application for managing Claude sessions and git workflows. Built with Tauri (Rust + React).

## Tech Stack

- **Frontend**: React 18 + TypeScript + Vite
- **Backend**: Rust + Tauri
- **Database**: SQLite + JSON files
- **Git Integration**: git2-rs (Rust) + simple-git (Node.js - future)
- **Build**: Cargo (Rust) + npm (Node.js)

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.70+ (install from https://rustup.rs/)
- Tauri CLI: `cargo install tauri-cli`

### Installation

```bash
# Install dependencies
npm install

# Install Rust dependencies (Tauri will do this automatically)
cd src-tauri
cargo build
```

### Development

```bash
# Start development server (Tauri + Vite)
npm run dev
```

This will:
1. Start the Vite dev server on localhost:1420
2. Launch the Tauri development window
3. Enable hot-reload for React code

### Build

```bash
# Build for production
npm run build

# Output binaries will be in src-tauri/target/release/
```

## Project Structure

```
claude-session-manager/
├── src/                        # React frontend
│   ├── components/             # Reusable React components
│   ├── App.tsx                 # Main app component
│   ├── main.tsx                # React entry point
│   └── index.css               # Global styles
│
├── src-tauri/                  # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs             # Tauri app entry point
│   │   ├── lib.rs              # Module exports
│   │   ├── commands.rs         # IPC command handlers
│   │   ├── session.rs          # Session data models
│   │   ├── db.rs               # Database operations
│   │   ├── git.rs              # Git operations
│   │   └── build.rs            # Build script
│   ├── Cargo.toml              # Rust dependencies
│   └── tauri.conf.json         # Tauri configuration
│
├── index.html                  # HTML entry point
├── vite.config.ts              # Vite configuration
├── tsconfig.json               # TypeScript config
├── package.json                # Node.js dependencies
├── README.md                   # Project README
└── .gitignore                  # Git ignore rules
```

## Architecture

### Frontend (React)
- Dashboard view with session list
- Session details viewer
- Git branch/worktree manager
- Project organizer sidebar

### Backend (Rust/Tauri)
- **Commands** (`commands.rs`): IPC endpoints called from React
- **Session Module** (`session.rs`): Session data structures
- **Database** (`db.rs`): SQLite operations for persistence
- **Git** (`git.rs`): Git worktree and branch operations

### Data Flow
1. React UI calls Tauri commands via `invoke()`
2. Tauri routes to Rust handler functions
3. Rust handles database/git operations
4. Results serialized to JSON and returned to React
5. React updates UI with new data

## IPC Commands

Available Tauri commands (defined in `commands.rs`):

- `get_sessions()` → `Vec<Session>`
- `create_session(name: String, project: Option<String>)` → `Session`
- `delete_session(id: String)` → `()`
- `update_session(id: String, name: String, tags: Option<Vec<String>>)` → `()`
- `get_git_branches(repo_path: String)` → `Vec<String>`
- `create_worktree(repo_path: String, branch: String, path: String)` → `()`
- `delete_worktree(path: String)` → `()`

## Database Schema

### sessions table
```sql
CREATE TABLE sessions (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  project TEXT,
  context TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  tags TEXT
);
```

### projects table
```sql
CREATE TABLE projects (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  path TEXT NOT NULL,
  created_at TEXT NOT NULL
);
```

## Development Workflow

### Adding a New Session Command

1. **Define the command in `commands.rs`:**
   ```rust
   #[tauri::command]
   pub async fn my_command(param: String) -> Result<String, String> {
       // Implementation
       Ok("result".to_string())
   }
   ```

2. **Register in `main.rs`:**
   ```rust
   .invoke_handler(tauri::generate_handler![
       commands::my_command,
   ])
   ```

3. **Call from React:**
   ```tsx
   import { invoke } from "@tauri-apps/api/tauri";
   
   const result = await invoke("my_command", { param: "value" });
   ```

### Adding a New React Component

1. Create component in `src/components/MyComponent.tsx`
2. Export from `src/App.tsx`
3. Use in app with hot-reload

## Testing

Currently no automated tests configured. To add:

1. **Rust tests**: Add `#[cfg(test)]` modules in Rust files
2. **React tests**: Use Vitest + React Testing Library

Run tests with:
```bash
# Rust tests
cargo test

# React tests (future)
npm run test
```

## Common Issues

### Port Already in Use (1420)
If port 1420 is in use, edit `vite.config.ts`:
```ts
server: {
  port: 3000,  // Change to another port
}
```

### Tauri CLI Not Found
```bash
cargo install tauri-cli@1.6
```

### Rust Compilation Errors
Ensure you have the latest stable Rust:
```bash
rustup update
```

## Deployment

### Build Release Binaries

```bash
npm run build
```

Generates installers in `src-tauri/target/release/bundle/`:
- **Windows**: `.msi` installer
- **macOS**: `.app` bundle or `.dmg`
- **Linux**: `.AppImage` or `.deb`

### Cross-Platform Build

Building for multiple platforms requires:
- **Windows**: Visual Studio Build Tools or full VS
- **macOS**: Xcode Command Line Tools (`xcode-select --install`)
- **Linux**: Build essentials (`sudo apt-get install build-essential`)

## Next Steps (Implementation Phases)

### Phase 1: Core Session Storage ✅
- [x] Project scaffolding
- [ ] Implement SQLite database initialization
- [ ] Create Session CRUD operations
- [ ] Implement file watcher for session changes
- [ ] Serialize/deserialize session context

### Phase 2: Frontend Dashboard
- [ ] Session list view with filtering/search
- [ ] Session detail viewer
- [ ] Session tagging UI
- [ ] Project sidebar navigation

### Phase 3: Git Integration
- [ ] Wrap git2-rs for worktree operations
- [ ] Branch creation/deletion UI
- [ ] Worktree listing and management
- [ ] Cross-platform testing

### Phase 4: Testing & Polish
- [ ] Unit tests for database operations
- [ ] Integration tests for git commands
- [ ] E2E testing with Tauri
- [ ] Build and package for all platforms

## Resources

- [Tauri Documentation](https://tauri.app)
- [React Documentation](https://react.dev)
- [Rust Book](https://doc.rust-lang.org/book/)
- [git2-rs Documentation](https://docs.rs/git2/)

## Contributing

1. Create a feature branch: `git checkout -b feature/your-feature`
2. Make changes and commit
3. Push to GitHub: `git push origin feature/your-feature`
4. Create a Pull Request

## License

MIT
