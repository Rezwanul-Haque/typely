use crate::domain::{SnippetRepository, DomainEvent};
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteSnippetService {
    repository: Arc<dyn SnippetRepository>,
}

impl DeleteSnippetService {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: Uuid) -> Result<bool> {
        // Find the snippet first to get its trigger for logging
        let snippet = self.repository.find_by_id(&id).await?;
        
        if let Some(snippet) = snippet {
            // Delete the snippet
            let deleted = self.repository.delete(&id).await?;
            
            if deleted {
                // Log domain event
                let event = DomainEvent::SnippetDeleted {
                    snippet_id: id,
                    trigger: snippet.trigger,
                    timestamp: chrono::Utc::now(),
                };
                log::info!("Snippet deleted: {:?}", event);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            // Snippet doesn't exist
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::services::CreateSnippetService;
    use crate::app::dto::CreateSnippetRequest;
    use crate::infra::{DatabaseConnection, SqliteSnippetRepository};
    use tempfile::TempDir;

    async fn create_test_use_case() -> (DeleteSnippetService, CreateSnippetService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = Arc::new(SqliteSnippetRepository::new(db_connection.pool().clone()));
        let delete_use_case = DeleteSnippetService::new(repository.clone());
        let create_use_case = CreateSnippetService::new(repository);
        (delete_use_case, create_use_case, temp_dir)
    }

    #[tokio::test]
    async fn test_delete_existing_snippet() {
        let (delete_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create a snippet first
        let create_request = CreateSnippetRequest {
            trigger: "::test".to_string(),
            replacement: "Test".to_string(),
            tags: None,
        };
        let created = create_use_case.execute(create_request).await.unwrap();

        // Delete the snippet
        let deleted = delete_use_case.execute(created.id).await.unwrap();
        assert!(deleted);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_snippet() {
        let (delete_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;
        
        let deleted = delete_use_case.execute(Uuid::new_v4()).await.unwrap();
        assert!(!deleted);
    }
}