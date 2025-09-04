use crate::app::dto::{CreateSnippetRequest, SnippetDto};
use crate::domain::{DomainEvent, Snippet, SnippetRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct CreateSnippetService {
    repository: Arc<dyn SnippetRepository>,
}

impl CreateSnippetService {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: CreateSnippetRequest) -> Result<SnippetDto> {
        // Validate that the trigger doesn't already exist
        if self
            .repository
            .exists_with_trigger(&request.trigger)
            .await?
        {
            return Err(anyhow::anyhow!(
                "A snippet with trigger '{}' already exists",
                request.trigger
            ));
        }

        // Create the snippet
        let mut snippet = Snippet::new(request.trigger.clone(), request.replacement)?;

        // Add tags if provided
        if let Some(tags) = request.tags {
            for tag in tags {
                snippet.add_tag(tag);
            }
        }

        // Save to repository
        self.repository.save(&snippet).await?;

        // Log domain event
        let event = DomainEvent::SnippetCreated {
            snippet_id: snippet.id,
            trigger: snippet.trigger.clone(),
            timestamp: chrono::Utc::now(),
        };
        log::info!("Snippet created: {:?}", event);

        Ok(SnippetDto::from(snippet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::{DatabaseConnection, SqliteSnippetRepository};
    use tempfile::TempDir;

    async fn create_test_use_case() -> (CreateSnippetService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = Arc::new(SqliteSnippetRepository::new(db_connection.pool().clone()));
        let use_case = CreateSnippetService::new(repository);
        (use_case, temp_dir)
    }

    #[tokio::test]
    async fn test_create_snippet_success() {
        let (use_case, _temp_dir) = create_test_use_case().await;

        let request = CreateSnippetRequest {
            trigger: "::hello".to_string(),
            replacement: "Hello, World!".to_string(),
            tags: Some(vec!["greeting".to_string()]),
        };

        let result = use_case.execute(request).await.unwrap();

        assert_eq!(result.trigger, "::hello");
        assert_eq!(result.replacement, "Hello, World!");
        assert!(result.tags.contains(&"greeting".to_string()));
        assert!(result.is_active);
        assert_eq!(result.usage_count, 0);
    }

    #[tokio::test]
    async fn test_create_snippet_duplicate_trigger() {
        let (use_case, _temp_dir) = create_test_use_case().await;

        let request1 = CreateSnippetRequest {
            trigger: "::test".to_string(),
            replacement: "First".to_string(),
            tags: None,
        };

        let request2 = CreateSnippetRequest {
            trigger: "::test".to_string(),
            replacement: "Second".to_string(),
            tags: None,
        };

        // First creation should succeed
        let result1 = use_case.execute(request1).await;
        assert!(result1.is_ok());

        // Second creation should fail
        let result2 = use_case.execute(request2).await;
        assert!(result2.is_err());
        assert!(result2.unwrap_err().to_string().contains("already exists"));
    }

    #[tokio::test]
    async fn test_create_snippet_invalid_trigger() {
        let (use_case, _temp_dir) = create_test_use_case().await;

        let request = CreateSnippetRequest {
            trigger: "".to_string(), // Empty trigger
            replacement: "Test".to_string(),
            tags: None,
        };

        let result = use_case.execute(request).await;
        assert!(result.is_err());
    }
}
