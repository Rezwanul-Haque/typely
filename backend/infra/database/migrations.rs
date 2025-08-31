use sqlx::{Pool, Sqlite};
use anyhow::Result;

pub struct MigrationRunner {
    pool: Pool<Sqlite>,
}

impl MigrationRunner {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn run_migrations(&self) -> Result<()> {
        // Create migrations table to track applied migrations
        self.create_migrations_table().await?;

        // Apply migrations in order
        self.apply_migration_001().await?;
        self.apply_migration_002().await?;
        self.apply_migration_003().await?;

        Ok(())
    }

    async fn create_migrations_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn check_and_record_migration(&self, name: &str) -> Result<bool> {
        // Check if migration has already been applied
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM migrations WHERE name = ?",
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        if existing > 0 {
            return Ok(false); // Already applied
        }

        Ok(true) // Should apply
    }

    async fn record_migration(&self, name: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO migrations (name, applied_at) VALUES (?, datetime('now'))",
        )
        .bind(name)
        .execute(&self.pool)
        .await?;

        log::info!("Applied migration: {}", name);
        Ok(())
    }

    async fn apply_migration_001(&self) -> Result<()> {
        if !self.check_and_record_migration("001_create_snippets").await? {
            return Ok(());
        }

        Self::migration_001_create_snippets(&self.pool).await?;
        self.record_migration("001_create_snippets").await?;
        Ok(())
    }

    async fn apply_migration_002(&self) -> Result<()> {
        if !self.check_and_record_migration("002_add_indexes").await? {
            return Ok(());
        }

        Self::migration_002_add_indexes(&self.pool).await?;
        self.record_migration("002_add_indexes").await?;
        Ok(())
    }

    async fn apply_migration_003(&self) -> Result<()> {
        if !self.check_and_record_migration("003_create_events").await? {
            return Ok(());
        }

        Self::migration_003_create_events(&self.pool).await?;
        self.record_migration("003_create_events").await?;
        Ok(())
    }

    async fn migration_001_create_snippets(pool: &Pool<Sqlite>) -> Result<()> {
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
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn migration_002_add_indexes(pool: &Pool<Sqlite>) -> Result<()> {
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_snippets_trigger ON snippets(trigger)
            "#,
        )
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_snippets_is_active ON snippets(is_active)
            "#,
        )
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_snippets_usage_count ON snippets(usage_count)
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn migration_003_create_events(pool: &Pool<Sqlite>) -> Result<()> {
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
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_events_snippet_id ON events(snippet_id)
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}