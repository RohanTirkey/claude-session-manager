use rusqlite::{Connection, Result as SqliteResult, params};
use std::path::PathBuf;
use anyhow::Result;
use crate::session::{Session, Project};
use chrono::Utc;
use uuid::Uuid;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                project TEXT,
                context TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                tags TEXT
            );
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL
            );"
        )?;
        Ok(Database { conn })
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    // Session operations
    pub fn create_session(&self, name: String, project: Option<String>) -> Result<Session> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO sessions (id, name, project, created_at, updated_at, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id, &name, &project, &now, &now, "[]"],
        )?;

        Ok(Session {
            id,
            name,
            project,
            context: None,
            created_at: now.clone(),
            updated_at: now,
            tags: Some(vec![]),
        })
    }

    pub fn get_session(&self, id: &str) -> Result<Option<Session>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, project, context, created_at, updated_at, tags FROM sessions WHERE id = ?1"
        )?;

        let session = stmt.query_row(params![id], |row| {
            let tags_json: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();

            Ok(Session {
                id: row.get(0)?,
                name: row.get(1)?,
                project: row.get(2)?,
                context: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                tags: Some(tags),
            })
        }).optional()?;

        Ok(session)
    }

    pub fn get_all_sessions(&self) -> Result<Vec<Session>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, project, context, created_at, updated_at, tags FROM sessions ORDER BY updated_at DESC"
        )?;

        let sessions = stmt.query_map([], |row| {
            let tags_json: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();

            Ok(Session {
                id: row.get(0)?,
                name: row.get(1)?,
                project: row.get(2)?,
                context: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                tags: Some(tags),
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    pub fn update_session(&self, id: String, name: String, tags: Option<Vec<String>>) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let tags_json = serde_json::to_string(&tags.unwrap_or_default())?;

        self.conn.execute(
            "UPDATE sessions SET name = ?1, tags = ?2, updated_at = ?3 WHERE id = ?4",
            params![&name, &tags_json, &now, &id],
        )?;

        Ok(())
    }

    pub fn delete_session(&self, id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM sessions WHERE id = ?1",
            params![id],
        )?;

        Ok(())
    }

    pub fn save_session_context(&self, id: &str, context: String) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "UPDATE sessions SET context = ?1, updated_at = ?2 WHERE id = ?3",
            params![context, now, id],
        )?;

        Ok(())
    }

    pub fn get_sessions_by_project(&self, project: &str) -> Result<Vec<Session>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, project, context, created_at, updated_at, tags FROM sessions WHERE project = ?1 ORDER BY updated_at DESC"
        )?;

        let sessions = stmt.query_map(params![project], |row| {
            let tags_json: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();

            Ok(Session {
                id: row.get(0)?,
                name: row.get(1)?,
                project: row.get(2)?,
                context: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                tags: Some(tags),
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    // Project operations
    pub fn create_project(&self, name: String, path: String) -> Result<Project> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO projects (id, name, path, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![&id, &name, &path, &now],
        )?;

        Ok(Project {
            id,
            name,
            path,
            created_at: now,
        })
    }

    pub fn get_all_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, path, created_at FROM projects ORDER BY created_at DESC"
        )?;

        let projects = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    pub fn delete_project(&self, id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM projects WHERE id = ?1",
            params![id],
        )?;

        Ok(())
    }
}
