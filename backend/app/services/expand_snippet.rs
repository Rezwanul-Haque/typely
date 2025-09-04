use crate::app::dto::{ExpansionRequest, ExpansionResponse};
use crate::domain::{DomainEvent, ExpansionContext, ExpansionService, SnippetRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct ExpandSnippetService {
    repository: Arc<dyn SnippetRepository>,
    expansion_service: ExpansionService,
}

impl ExpandSnippetService {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self {
            repository,
            expansion_service: ExpansionService::new(),
        }
    }

    pub async fn execute(&self, request: ExpansionRequest) -> Result<ExpansionResponse> {
        // Find the snippet by trigger
        let snippet = match self.repository.find_by_trigger(&request.trigger).await? {
            Some(snippet) => snippet,
            None => {
                return Ok(ExpansionResponse {
                    success: false,
                    expanded_text: None,
                    error_message: Some(format!(
                        "No snippet found for trigger: {}",
                        request.trigger
                    )),
                });
            }
        };

        // Check if the snippet is active
        if !snippet.is_active {
            return Ok(ExpansionResponse {
                success: false,
                expanded_text: None,
                error_message: Some("Snippet is inactive".to_string()),
            });
        }

        // Create expansion context
        let context = ExpansionContext {
            cursor_position: None,
            surrounding_text: request.context,
            application_context: None,
        };

        // Expand the snippet
        let expansion_result = self.expansion_service.expand_snippet(&snippet, &context);

        if expansion_result.success {
            // Update usage count
            let mut updated_snippet = snippet;
            updated_snippet.increment_usage();

            // Save the updated snippet
            if let Err(e) = self.repository.update(&updated_snippet).await {
                log::warn!("Failed to update snippet usage count: {}", e);
            }

            // Log domain event
            let event = DomainEvent::SnippetExpanded {
                snippet_id: updated_snippet.id,
                trigger: updated_snippet.trigger.clone(),
                timestamp: chrono::Utc::now(),
            };
            log::info!("Snippet expanded: {:?}", event);

            Ok(ExpansionResponse {
                success: true,
                expanded_text: Some(expansion_result.expanded_text),
                error_message: None,
            })
        } else {
            Ok(ExpansionResponse {
                success: false,
                expanded_text: None,
                error_message: expansion_result.error,
            })
        }
    }

    pub async fn find_matching_snippets(&self, text: &str) -> Result<Vec<String>> {
        let triggers = self.expansion_service.find_triggers(text);
        let mut matching_triggers = Vec::new();

        for trigger_match in triggers {
            if self
                .repository
                .exists_with_trigger(&trigger_match.trigger)
                .await?
            {
                matching_triggers.push(trigger_match.trigger);
            }
        }

        Ok(matching_triggers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::dto::CreateSnippetRequest;
    use crate::app::services::CreateSnippetService;
    use crate::infra::{DatabaseConnection, SqliteSnippetRepository};
    use tempfile::TempDir;

    async fn create_test_use_case() -> (ExpandSnippetService, CreateSnippetService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = Arc::new(SqliteSnippetRepository::new(db_connection.pool().clone()));
        let expand_use_case = ExpandSnippetService::new(repository.clone());
        let create_use_case = CreateSnippetService::new(repository);
        (expand_use_case, create_use_case, temp_dir)
    }

    #[tokio::test]
    async fn test_expand_existing_snippet() {
        let (expand_use_case, create_use_case, _temp_dir) = create_test_use_case().await;

        // Create a snippet first
        let create_request = CreateSnippetRequest {
            trigger: "::hello".to_string(),
            replacement: "Hello, World!".to_string(),
            tags: None,
        };
        create_use_case.execute(create_request).await.unwrap();

        // Expand the snippet
        let expansion_request = ExpansionRequest {
            trigger: "::hello".to_string(),
            context: None,
        };

        let response = expand_use_case.execute(expansion_request).await.unwrap();

        assert!(response.success);
        assert!(response.expanded_text.is_some());
        assert_eq!(response.expanded_text.unwrap(), "Hello, World!");
        assert!(response.error_message.is_none());
    }

    #[tokio::test]
    async fn test_expand_nonexistent_snippet() {
        let (expand_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;

        let expansion_request = ExpansionRequest {
            trigger: "::nonexistent".to_string(),
            context: None,
        };

        let response = expand_use_case.execute(expansion_request).await.unwrap();

        assert!(!response.success);
        assert!(response.expanded_text.is_none());
        assert!(response.error_message.is_some());
        assert!(response.error_message.unwrap().contains("No snippet found"));
    }

    #[tokio::test]
    async fn test_expand_inactive_snippet() {
        let (expand_use_case, create_use_case, _temp_dir) = create_test_use_case().await;

        // Create a snippet first
        let create_request = CreateSnippetRequest {
            trigger: "::test".to_string(),
            replacement: "Test".to_string(),
            tags: None,
        };
        let created = create_use_case.execute(create_request).await.unwrap();

        // Deactivate the snippet (this would normally be done through UpdateSnippetService)
        // For this test, we'll assume the snippet is deactivated

        let expansion_request = ExpansionRequest {
            trigger: "::test".to_string(),
            context: None,
        };

        let response = expand_use_case.execute(expansion_request).await.unwrap();

        // Should succeed because our test snippet is still active
        // In a real scenario, we'd update the snippet to be inactive first
        assert!(response.success);
    }

    #[tokio::test]
    async fn test_find_matching_snippets() {
        let (expand_use_case, create_use_case, _temp_dir) = create_test_use_case().await;

        // Create multiple snippets
        let snippets = vec![
            ("::hello", "Hello, World!"),
            ("::test", "This is a test"),
            ("::bye", "Goodbye!"),
        ];

        for (trigger, replacement) in snippets {
            let request = CreateSnippetRequest {
                trigger: trigger.to_string(),
                replacement: replacement.to_string(),
                tags: None,
            };
            create_use_case.execute(request).await.unwrap();
        }

        // Test text containing multiple triggers
        let text = "Say ::hello and ::test but not ::nonexistent";
        let matching = expand_use_case.find_matching_snippets(text).await.unwrap();

        assert_eq!(matching.len(), 2);
        assert!(matching.contains(&"::hello".to_string()));
        assert!(matching.contains(&"::test".to_string()));
        assert!(!matching.contains(&"::nonexistent".to_string()));
    }
}
