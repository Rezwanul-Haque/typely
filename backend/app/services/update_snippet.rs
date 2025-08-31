use crate::app::dto::{UpdateSnippetRequest, SnippetDto};
use crate::domain::{SnippetRepository, DomainEvent};
use anyhow::Result;
use std::sync::Arc;

pub struct UpdateSnippetService {
    repository: Arc<dyn SnippetRepository>,
}

impl UpdateSnippetService {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: UpdateSnippetRequest) -> Result<SnippetDto> {
        // Find the existing snippet
        let mut snippet = self.repository.find_by_id(&request.id).await?
            .ok_or_else(|| anyhow::anyhow!("Snippet not found"))?;

        // Update trigger if provided
        if let Some(new_trigger) = request.trigger {
            if new_trigger != snippet.trigger {
                // Check if the new trigger already exists
                if self.repository.exists_with_trigger(&new_trigger).await? {
                    return Err(anyhow::anyhow!("A snippet with trigger '{}' already exists", new_trigger));
                }
                snippet.update_trigger(new_trigger)?;
            }
        }

        // Update replacement if provided
        if let Some(new_replacement) = request.replacement {
            snippet.update_replacement(new_replacement)?;
        }

        // Update tags if provided
        if let Some(new_tags) = request.tags {
            // Clear existing tags and add new ones
            snippet.tags.clear();
            for tag in new_tags {
                snippet.add_tag(tag);
            }
        }

        // Update active status if provided
        if let Some(is_active) = request.is_active {
            if is_active && !snippet.is_active {
                snippet.activate();
            } else if !is_active && snippet.is_active {
                snippet.deactivate();
            }
        }

        // Save to repository
        self.repository.update(&snippet).await?;

        // Log domain event
        let event = DomainEvent::SnippetUpdated {
            snippet_id: snippet.id,
            trigger: snippet.trigger.clone(),
            timestamp: chrono::Utc::now(),
        };
        log::info!("Snippet updated: {:?}", event);

        Ok(SnippetDto::from(snippet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::services::CreateSnippetService;
    use crate::app::dto::CreateSnippetRequest;
    use crate::infra::{DatabaseConnection, SqliteSnippetRepository};
    use tempfile::TempDir;

    async fn create_test_use_case() -> (UpdateSnippetService, CreateSnippetService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = Arc::new(SqliteSnippetRepository::new(db_connection.pool().clone()));
        let update_use_case = UpdateSnippetService::new(repository.clone());
        let create_use_case = CreateSnippetService::new(repository);
        (update_use_case, create_use_case, temp_dir)
    }

    #[tokio::test]
    async fn test_update_snippet_replacement() {
        let (update_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create a snippet first
        let create_request = CreateSnippetRequest {
            trigger: "::test".to_string(),
            replacement: "Original".to_string(),
            tags: None,
        };
        let created = create_use_case.execute(create_request).await.unwrap();

        // Update the replacement
        let update_request = UpdateSnippetRequest {
            id: created.id,
            trigger: None,
            replacement: Some("Updated replacement".to_string()),
            tags: None,
            is_active: None,
        };
        
        let updated = update_use_case.execute(update_request).await.unwrap();
        
        assert_eq!(updated.replacement, "Updated replacement");
        assert_eq!(updated.trigger, "::test"); // Should remain unchanged
    }

    #[tokio::test]
    async fn test_update_snippet_trigger() {
        let (update_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create a snippet first
        let create_request = CreateSnippetRequest {
            trigger: "::old".to_string(),
            replacement: "Test".to_string(),
            tags: None,
        };
        let created = create_use_case.execute(create_request).await.unwrap();

        // Update the trigger
        let update_request = UpdateSnippetRequest {
            id: created.id,
            trigger: Some("::new".to_string()),
            replacement: None,
            tags: None,
            is_active: None,
        };
        
        let updated = update_use_case.execute(update_request).await.unwrap();
        
        assert_eq!(updated.trigger, "::new");
        assert_eq!(updated.replacement, "Test"); // Should remain unchanged
    }

    #[tokio::test]
    async fn test_update_snippet_deactivate() {
        let (update_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create a snippet first
        let create_request = CreateSnippetRequest {
            trigger: "::test".to_string(),
            replacement: "Test".to_string(),
            tags: None,
        };
        let created = create_use_case.execute(create_request).await.unwrap();
        assert!(created.is_active);

        // Deactivate the snippet
        let update_request = UpdateSnippetRequest {
            id: created.id,
            trigger: None,
            replacement: None,
            tags: None,
            is_active: Some(false),
        };
        
        let updated = update_use_case.execute(update_request).await.unwrap();
        assert!(!updated.is_active);
    }

    #[tokio::test]
    async fn test_update_nonexistent_snippet() {
        let (update_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;
        
        let update_request = UpdateSnippetRequest {
            id: uuid::Uuid::new_v4(),
            trigger: None,
            replacement: Some("Test".to_string()),
            tags: None,
            is_active: None,
        };
        
        let result = update_use_case.execute(update_request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}