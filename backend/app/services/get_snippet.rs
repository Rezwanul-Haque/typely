use crate::app::dto::SnippetDto;
use crate::domain::SnippetRepository;
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetSnippetService {
    repository: Arc<dyn SnippetRepository>,
}

impl GetSnippetService {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<Option<SnippetDto>> {
        let snippet = self.repository.find_by_id(&id).await?;
        Ok(snippet.map(SnippetDto::from))
    }

    pub async fn execute_by_trigger(&self, trigger: &str) -> Result<Option<SnippetDto>> {
        let snippet = self.repository.find_by_trigger(trigger).await?;
        Ok(snippet.map(SnippetDto::from))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::dto::CreateSnippetRequest;
    use crate::app::services::CreateSnippetService;
    use crate::infra::{DatabaseConnection, SqliteSnippetRepository};
    use tempfile::TempDir;

    async fn create_test_use_case() -> (GetSnippetService, CreateSnippetService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = Arc::new(SqliteSnippetRepository::new(db_connection.pool().clone()));
        let get_use_case = GetSnippetService::new(repository.clone());
        let create_use_case = CreateSnippetService::new(repository);
        (get_use_case, create_use_case, temp_dir)
    }

    #[tokio::test]
    async fn test_get_snippet_by_id() {
        let (get_use_case, create_use_case, _temp_dir) = create_test_use_case().await;

        // Create a snippet first
        let create_request = CreateSnippetRequest {
            trigger: "::test".to_string(),
            replacement: "Test replacement".to_string(),
            tags: Some(vec!["test".to_string()]),
        };
        let created = create_use_case.execute(create_request).await.unwrap();

        // Get the snippet by ID
        let found = get_use_case.execute(created.id).await.unwrap();

        assert!(found.is_some());
        let snippet = found.unwrap();
        assert_eq!(snippet.id, created.id);
        assert_eq!(snippet.trigger, "::test");
        assert_eq!(snippet.replacement, "Test replacement");
    }

    #[tokio::test]
    async fn test_get_snippet_by_trigger() {
        let (get_use_case, create_use_case, _temp_dir) = create_test_use_case().await;

        // Create a snippet first
        let create_request = CreateSnippetRequest {
            trigger: "::hello".to_string(),
            replacement: "Hello, World!".to_string(),
            tags: None,
        };
        let created = create_use_case.execute(create_request).await.unwrap();

        // Get the snippet by trigger
        let found = get_use_case.execute_by_trigger("::hello").await.unwrap();

        assert!(found.is_some());
        let snippet = found.unwrap();
        assert_eq!(snippet.id, created.id);
        assert_eq!(snippet.trigger, "::hello");
        assert_eq!(snippet.replacement, "Hello, World!");
    }

    #[tokio::test]
    async fn test_get_nonexistent_snippet() {
        let (get_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;

        // Try to get a nonexistent snippet by ID
        let found = get_use_case.execute(Uuid::new_v4()).await.unwrap();
        assert!(found.is_none());

        // Try to get a nonexistent snippet by trigger
        let found = get_use_case
            .execute_by_trigger("::nonexistent")
            .await
            .unwrap();
        assert!(found.is_none());
    }
}
