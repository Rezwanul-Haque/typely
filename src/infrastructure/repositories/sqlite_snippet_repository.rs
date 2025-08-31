use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::{Pool, Sqlite, Row};
use uuid::Uuid;

use crate::domain::{Snippet, SnippetRepository, SnippetQuery, SortBy, SortOrder};

pub struct SqliteSnippetRepository {
    pool: Pool<Sqlite>,
}

impl SqliteSnippetRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SnippetRepository for SqliteSnippetRepository {
    async fn save(&self, snippet: &Snippet) -> anyhow::Result<()> {
        let tags_json = serde_json::to_string(&snippet.tags)?;
        
        sqlx::query(
            r#"
            INSERT INTO snippets (
                id, trigger, replacement, created_at, updated_at, 
                is_active, usage_count, tags
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(snippet.id.to_string())
        .bind(&snippet.trigger)
        .bind(&snippet.replacement)
        .bind(snippet.created_at.to_rfc3339())
        .bind(snippet.updated_at.to_rfc3339())
        .bind(snippet.is_active as i64)
        .bind(snippet.usage_count as i64)
        .bind(tags_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Snippet>> {
        let row = sqlx::query(
            r#"
            SELECT id, trigger, replacement, created_at, updated_at, 
                   is_active, usage_count, tags
            FROM snippets 
            WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let snippet = self.row_to_snippet(row)?;
                Ok(Some(snippet))
            }
            None => Ok(None),
        }
    }

    async fn find_by_trigger(&self, trigger: &str) -> anyhow::Result<Option<Snippet>> {
        let row = sqlx::query(
            r#"
            SELECT id, trigger, replacement, created_at, updated_at, 
                   is_active, usage_count, tags
            FROM snippets 
            WHERE trigger = ?
            "#,
        )
        .bind(trigger)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let snippet = self.row_to_snippet(row)?;
                Ok(Some(snippet))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self, query: &SnippetQuery) -> anyhow::Result<Vec<Snippet>> {
        let mut sql = String::from(
            r#"
            SELECT id, trigger, replacement, created_at, updated_at, 
                   is_active, usage_count, tags
            FROM snippets 
            WHERE 1=1
            "#,
        );

        let mut bind_values: Vec<String> = Vec::new();

        // Add filters
        if let Some(is_active) = query.is_active {
            sql.push_str(" AND is_active = ?");
            bind_values.push((is_active as i64).to_string());
        }

        if let Some(ref search_term) = query.search_term {
            sql.push_str(" AND (trigger LIKE ? OR replacement LIKE ?)");
            let search_pattern = format!("%{}%", search_term);
            bind_values.push(search_pattern.clone());
            bind_values.push(search_pattern);
        }

        // Add sorting
        match query.sort_by {
            SortBy::CreatedAt => sql.push_str(" ORDER BY created_at"),
            SortBy::UpdatedAt => sql.push_str(" ORDER BY updated_at"),
            SortBy::UsageCount => sql.push_str(" ORDER BY usage_count"),
            SortBy::Trigger => sql.push_str(" ORDER BY trigger"),
        }

        match query.sort_order {
            SortOrder::Ascending => sql.push_str(" ASC"),
            SortOrder::Descending => sql.push_str(" DESC"),
        }

        // Add pagination
        if let Some(limit) = query.limit {
            sql.push_str(" LIMIT ?");
            bind_values.push(limit.to_string());
        }

        if let Some(offset) = query.offset {
            sql.push_str(" OFFSET ?");
            bind_values.push(offset.to_string());
        }

        let mut query_builder = sqlx::query(&sql);
        for value in bind_values {
            query_builder = query_builder.bind(value);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        let mut snippets = Vec::new();
        for row in rows {
            let snippet = self.row_to_snippet(row)?;
            snippets.push(snippet);
        }

        Ok(snippets)
    }

    async fn update(&self, snippet: &Snippet) -> anyhow::Result<()> {
        let tags_json = serde_json::to_string(&snippet.tags)?;
        
        sqlx::query(
            r#"
            UPDATE snippets SET 
                trigger = ?, replacement = ?, updated_at = ?, 
                is_active = ?, usage_count = ?, tags = ?
            WHERE id = ?
            "#,
        )
        .bind(&snippet.trigger)
        .bind(&snippet.replacement)
        .bind(snippet.updated_at.to_rfc3339())
        .bind(snippet.is_active as i64)
        .bind(snippet.usage_count as i64)
        .bind(tags_json)
        .bind(snippet.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> anyhow::Result<bool> {
        let result = sqlx::query("DELETE FROM snippets WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn count(&self, query: &SnippetQuery) -> anyhow::Result<u64> {
        let mut sql = String::from("SELECT COUNT(*) FROM snippets WHERE 1=1");
        let mut bind_values: Vec<String> = Vec::new();

        // Add filters
        if let Some(is_active) = query.is_active {
            sql.push_str(" AND is_active = ?");
            bind_values.push((is_active as i64).to_string());
        }

        if let Some(ref search_term) = query.search_term {
            sql.push_str(" AND (trigger LIKE ? OR replacement LIKE ?)");
            let search_pattern = format!("%{}%", search_term);
            bind_values.push(search_pattern.clone());
            bind_values.push(search_pattern);
        }

        let mut query_builder = sqlx::query_scalar::<_, i64>(&sql);
        for value in bind_values {
            query_builder = query_builder.bind(value);
        }

        let count = query_builder.fetch_one(&self.pool).await?;
        Ok(count as u64)
    }

    async fn exists_with_trigger(&self, trigger: &str) -> anyhow::Result<bool> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM snippets WHERE trigger = ?",
        )
        .bind(trigger)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }
}

impl SqliteSnippetRepository {
    fn row_to_snippet(&self, row: sqlx::sqlite::SqliteRow) -> anyhow::Result<Snippet> {
        let id_str: String = row.get("id");
        let id = Uuid::parse_str(&id_str)?;
        
        let created_at_str: String = row.get("created_at");
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
        
        let updated_at_str: String = row.get("updated_at");
        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);
        
        let is_active_int: i64 = row.get("is_active");
        let is_active = is_active_int != 0;
        
        let usage_count_int: i64 = row.get("usage_count");
        let usage_count = usage_count_int as u64;
        
        let tags_json: String = row.get("tags");
        let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
        
        Ok(Snippet {
            id,
            trigger: row.get("trigger"),
            replacement: row.get("replacement"),
            created_at,
            updated_at,
            is_active,
            usage_count,
            tags,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::DatabaseConnection;
    use tempfile::TempDir;

    async fn create_test_repository() -> (SqliteSnippetRepository, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = SqliteSnippetRepository::new(db_connection.pool().clone());
        (repository, temp_dir)
    }

    #[tokio::test]
    async fn test_save_and_find_by_id() {
        let (repository, _temp_dir) = create_test_repository().await;
        
        let snippet = Snippet::new("::test".to_string(), "Test replacement".to_string()).unwrap();
        let snippet_id = snippet.id;
        
        repository.save(&snippet).await.unwrap();
        
        let found = repository.find_by_id(&snippet_id).await.unwrap();
        assert!(found.is_some());
        
        let found_snippet = found.unwrap();
        assert_eq!(found_snippet.trigger, "::test");
        assert_eq!(found_snippet.replacement, "Test replacement");
    }

    #[tokio::test]
    async fn test_find_by_trigger() {
        let (repository, _temp_dir) = create_test_repository().await;
        
        let snippet = Snippet::new("::hello".to_string(), "Hello World".to_string()).unwrap();
        repository.save(&snippet).await.unwrap();
        
        let found = repository.find_by_trigger("::hello").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().replacement, "Hello World");
        
        let not_found = repository.find_by_trigger("::notfound").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_update() {
        let (repository, _temp_dir) = create_test_repository().await;
        
        let mut snippet = Snippet::new("::test".to_string(), "Original".to_string()).unwrap();
        repository.save(&snippet).await.unwrap();
        
        snippet.update_replacement("Updated".to_string()).unwrap();
        repository.update(&snippet).await.unwrap();
        
        let found = repository.find_by_id(&snippet.id).await.unwrap().unwrap();
        assert_eq!(found.replacement, "Updated");
    }

    #[tokio::test]
    async fn test_delete() {
        let (repository, _temp_dir) = create_test_repository().await;
        
        let snippet = Snippet::new("::test".to_string(), "Test".to_string()).unwrap();
        let snippet_id = snippet.id;
        
        repository.save(&snippet).await.unwrap();
        
        let deleted = repository.delete(&snippet_id).await.unwrap();
        assert!(deleted);
        
        let found = repository.find_by_id(&snippet_id).await.unwrap();
        assert!(found.is_none());
    }
}