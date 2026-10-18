# Claude Session Manager

A cross-platform desktop application for managing Claude sessions and git workflows.

## Features

- **Session Persistence** - Save and restore Claude sessions across restarts
- **Session Browser** - View and organize session history
- **Git Integration** - Create and manage git branches and worktrees from the UI
- **Project Management** - Organize sessions by project
- **Cross-Platform** - Runs on Windows, macOS, and Linux

## Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri
- **Storage**: SQLite + JSON
- **Git Integration**: simple-git

## Getting Started

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Tauri CLI

### Installation

```bash
npm install
```

### Development

```bash
npm run dev
```

### Build

```bash
npm run build
```

## Project Structure

```
claude-session-manager/
├── src/                    # React frontend
│   ├── components/
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs
│   │   ├── db.rs
│   │   ├── session.rs
│   │   └── git.rs
│   └── Cargo.toml
├── index.html
├── vite.config.ts
└── package.json
```

## License

MIT
