use crate::application::dto::{ExportSnippetsRequest, ImportSnippetData};
use crate::domain::{SnippetRepository, SnippetQuery};
use anyhow::Result;
use std::sync::Arc;

pub struct ExportSnippetsUseCase {
    repository: Arc<dyn SnippetRepository>,
}

impl ExportSnippetsUseCase {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: ExportSnippetsRequest) -> Result<Vec<ImportSnippetData>> {
        // Build query based on request
        let mut query = SnippetQuery::new();
        
        // Set active filter
        if request.include_inactive {
            query = query.with_all();
        } else {
            query = query.with_active_only();
        }
        
        // Apply tags filter if provided
        if let Some(tags_filter) = request.tags_filter {
            query = query.with_tags(tags_filter);
        }
        
        // Remove pagination to get all matching snippets
        query.limit = None;
        query.offset = None;
        
        // Get all matching snippets
        let snippets = self.repository.find_all(&query).await?;
        
        // Convert to export format
        let export_data: Vec<ImportSnippetData> = snippets
            .into_iter()
            .map(|snippet| ImportSnippetData {
                trigger: snippet.trigger,
                replacement: snippet.replacement,
                tags: if snippet.tags.is_empty() {
                    None
                } else {
                    Some(snippet.tags)
                },
            })
            .collect();
        
        Ok(export_data)
    }

    pub async fn export_to_json(&self, request: ExportSnippetsRequest) -> Result<String> {
        let export_data = self.execute(request).await?;
        
        let json = serde_json::to_string_pretty(&export_data)
            .map_err(|e| anyhow::anyhow!("Failed to serialize to JSON: {}", e))?;
        
        Ok(json)
    }

    pub async fn export_all_to_json(&self) -> Result<String> {
        let request = ExportSnippetsRequest {
            include_inactive: true,
            tags_filter: None,
        };
        
        self.export_to_json(request).await
    }

    pub async fn export_active_to_json(&self) -> Result<String> {
        let request = ExportSnippetsRequest {
            include_inactive: false,
            tags_filter: None,
        };
        
        self.export_to_json(request).await
    }

    pub async fn export_by_tags_to_json(&self, tags: Vec<String>, include_inactive: bool) -> Result<String> {
        let request = ExportSnippetsRequest {
            include_inactive,
            tags_filter: Some(tags),
        };
        
        self.export_to_json(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::use_cases::CreateSnippetUseCase;
    use crate::application::dto::CreateSnippetRequest;
    use crate::infrastructure::{DatabaseConnection, SqliteSnippetRepository};
    use tempfile::TempDir;

    async fn create_test_use_case() -> (ExportSnippetsUseCase, CreateSnippetUseCase, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = Arc::new(SqliteSnippetRepository::new(db_connection.pool().clone()));
        let export_use_case = ExportSnippetsUseCase::new(repository.clone());
        let create_use_case = CreateSnippetUseCase::new(repository);
        (export_use_case, create_use_case, temp_dir)
    }

    #[tokio::test]
    async fn test_export_empty_database() {
        let (export_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;
        
        let request = ExportSnippetsRequest {
            include_inactive: true,
            tags_filter: None,
        };
        
        let result = export_use_case.execute(request).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_export_all_snippets() {
        let (export_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create test snippets
        let snippets_data = vec![
            ("::hello", "Hello, World!", Some(vec!["greeting".to_string()])),
            ("::test", "This is a test", Some(vec!["test".to_string()])),
            ("::bye", "Goodbye!", None),
        ];

        for (trigger, replacement, tags) in snippets_data {
            let request = CreateSnippetRequest {
                trigger: trigger.to_string(),
                replacement: replacement.to_string(),
                tags,
            };
            create_use_case.execute(request).await.unwrap();
        }

        let export_request = ExportSnippetsRequest {
            include_inactive: true,
            tags_filter: None,
        };
        
        let result = export_use_case.execute(export_request).await.unwrap();
        
        assert_eq!(result.len(), 3);
        
        // Verify exported data
        let triggers: Vec<String> = result.iter().map(|s| s.trigger.clone()).collect();
        assert!(triggers.contains(&"::hello".to_string()));
        assert!(triggers.contains(&"::test".to_string()));
        assert!(triggers.contains(&"::bye".to_string()));
    }

    #[tokio::test]
    async fn test_export_to_json() {
        let (export_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create a test snippet
        let request = CreateSnippetRequest {
            trigger: "::json".to_string(),
            replacement: "JSON test".to_string(),
            tags: Some(vec!["json".to_string(), "test".to_string()]),
        };
        create_use_case.execute(request).await.unwrap();

        let export_request = ExportSnippetsRequest {
            include_inactive: true,
            tags_filter: None,
        };
        
        let json_result = export_use_case.export_to_json(export_request).await.unwrap();
        
        // Verify it's valid JSON
        let parsed: Vec<ImportSnippetData> = serde_json::from_str(&json_result).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].trigger, "::json");
        assert_eq!(parsed[0].replacement, "JSON test");
        assert!(parsed[0].tags.as_ref().unwrap().contains(&"json".to_string()));
    }

    #[tokio::test]
    async fn test_export_active_only() {
        let (export_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create active snippets (all snippets are active by default)
        let request1 = CreateSnippetRequest {
            trigger: "::active1".to_string(),
            replacement: "Active 1".to_string(),
            tags: None,
        };
        let request2 = CreateSnippetRequest {
            trigger: "::active2".to_string(),
            replacement: "Active 2".to_string(),
            tags: None,
        };
        
        create_use_case.execute(request1).await.unwrap();
        create_use_case.execute(request2).await.unwrap();

        // Export active only
        let json_result = export_use_case.export_active_to_json().await.unwrap();
        
        let parsed: Vec<ImportSnippetData> = serde_json::from_str(&json_result).unwrap();
        assert_eq!(parsed.len(), 2);
    }

    #[tokio::test]
    async fn test_export_all_to_json() {
        let (export_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create a test snippet
        let request = CreateSnippetRequest {
            trigger: "::all".to_string(),
            replacement: "All test".to_string(),
            tags: None,
        };
        create_use_case.execute(request).await.unwrap();

        let json_result = export_use_case.export_all_to_json().await.unwrap();
        
        let parsed: Vec<ImportSnippetData> = serde_json::from_str(&json_result).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].trigger, "::all");
    }
}