use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::{Path, PathBuf};

pub struct DatabaseConnection {
    pool: Pool<Sqlite>,
    database_path: PathBuf,
}

impl DatabaseConnection {
    pub async fn new(database_path: &Path) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = database_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let database_url = format!("sqlite:{}?mode=rwc", database_path.display());

        let pool = SqlitePool::connect(&database_url).await?;

        let connection = Self {
            pool,
            database_path: database_path.to_path_buf(),
        };

        // Run migrations
        connection.migrate().await?;

        Ok(connection)
    }

    pub async fn new_in_memory() -> Result<Self> {
        let pool = SqlitePool::connect("sqlite::memory:").await?;

        let connection = Self {
            pool,
            database_path: PathBuf::from(":memory:"),
        };

        // Run migrations
        connection.migrate().await?;

        Ok(connection)
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }

    async fn migrate(&self) -> Result<()> {
        // Create snippets table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS snippets (
                id TEXT PRIMARY KEY NOT NULL,
                trigger TEXT NOT NULL UNIQUE,
                replacement TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                is_active INTEGER NOT NULL DEFAULT 1,
                usage_count INTEGER NOT NULL DEFAULT 0,
                tags TEXT DEFAULT '[]'
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for better performance
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_snippets_trigger ON snippets(trigger)
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_snippets_is_active ON snippets(is_active)
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_snippets_usage_count ON snippets(usage_count)
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create events table for domain events (optional, for audit trail)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY NOT NULL,
                event_type TEXT NOT NULL,
                snippet_id TEXT NOT NULL,
                event_data TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_events_snippet_id ON events(snippet_id)
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn close(self) {
        self.pool.close().await;
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1").fetch_one(&self.pool).await?;
        Ok(())
    }
}

pub fn get_default_database_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    let app_dir = home_dir.join(".typely");
    Ok(app_dir.join("snippets.db"))
}
