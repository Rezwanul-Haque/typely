use crate::app::dto::{ImportResult, ImportSnippetsRequest};
use crate::domain::{Snippet, SnippetRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct ImportSnippetsService {
    repository: Arc<dyn SnippetRepository>,
}

impl ImportSnippetsService {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: ImportSnippetsRequest) -> Result<ImportResult> {
        let mut imported_count = 0;
        let mut skipped_count = 0;
        let mut error_count = 0;
        let mut errors = Vec::new();

        for snippet_data in request.snippets {
            match self
                .import_single_snippet(&snippet_data, request.overwrite_existing)
                .await
            {
                Ok(ImportStatus::Imported) => imported_count += 1,
                Ok(ImportStatus::Skipped) => skipped_count += 1,
                Err(e) => {
                    error_count += 1;
                    errors.push(format!(
                        "Failed to import '{}': {}",
                        snippet_data.trigger, e
                    ));
                }
            }
        }

        Ok(ImportResult {
            imported_count,
            skipped_count,
            error_count,
            errors,
        })
    }

    async fn import_single_snippet(
        &self,
        snippet_data: &crate::app::dto::ImportSnippetData,
        overwrite_existing: bool,
    ) -> Result<ImportStatus> {
        // Check if snippet with trigger already exists
        let exists = self
            .repository
            .exists_with_trigger(&snippet_data.trigger)
            .await?;

        if exists && !overwrite_existing {
            return Ok(ImportStatus::Skipped);
        }

        // Create new snippet
        let mut snippet = Snippet::new(
            snippet_data.trigger.clone(),
            snippet_data.replacement.clone(),
        )?;

        // Add tags if provided
        if let Some(ref tags) = snippet_data.tags {
            for tag in tags {
                snippet.add_tag(tag.clone());
            }
        }

        if exists && overwrite_existing {
            // Find existing snippet and update it
            if let Some(existing) = self
                .repository
                .find_by_trigger(&snippet_data.trigger)
                .await?
            {
                let mut updated_snippet = existing;
                updated_snippet.update_replacement(snippet_data.replacement.clone())?;

                // Update tags if provided
                if let Some(ref tags) = snippet_data.tags {
                    updated_snippet.tags.clear();
                    for tag in tags {
                        updated_snippet.add_tag(tag.clone());
                    }
                }

                self.repository.update(&updated_snippet).await?;
            }
        } else {
            // Save new snippet
            self.repository.save(&snippet).await?;
        }

        Ok(ImportStatus::Imported)
    }

    pub async fn import_from_json(
        &self,
        json_data: &str,
        overwrite_existing: bool,
    ) -> Result<ImportResult> {
        let import_data: Vec<crate::app::dto::ImportSnippetData> = serde_json::from_str(json_data)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

        let request = ImportSnippetsRequest {
            snippets: import_data,
            overwrite_existing,
        };

        self.execute(request).await
    }
}

#[derive(Debug, PartialEq)]
enum ImportStatus {
    Imported,
    Skipped,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::dto::CreateSnippetRequest;
    use crate::app::dto::ImportSnippetData;
    use crate::app::services::CreateSnippetService;
    use crate::infra::{DatabaseConnection, SqliteSnippetRepository};
    use tempfile::TempDir;

    async fn create_test_use_case() -> (ImportSnippetsService, CreateSnippetService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = Arc::new(SqliteSnippetRepository::new(db_connection.pool().clone()));
        let import_use_case = ImportSnippetsService::new(repository.clone());
        let create_use_case = CreateSnippetService::new(repository);
        (import_use_case, create_use_case, temp_dir)
    }

    #[tokio::test]
    async fn test_import_new_snippets() {
        let (import_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;

        let snippets = vec![
            ImportSnippetData {
                trigger: "::hello".to_string(),
                replacement: "Hello, World!".to_string(),
                tags: Some(vec!["greeting".to_string()]),
            },
            ImportSnippetData {
                trigger: "::test".to_string(),
                replacement: "This is a test".to_string(),
                tags: None,
            },
        ];

        let request = ImportSnippetsRequest {
            snippets,
            overwrite_existing: false,
        };

        let result = import_use_case.execute(request).await.unwrap();

        assert_eq!(result.imported_count, 2);
        assert_eq!(result.skipped_count, 0);
        assert_eq!(result.error_count, 0);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_import_with_existing_snippets_no_overwrite() {
        let (import_use_case, create_use_case, _temp_dir) = create_test_use_case().await;

        // Create an existing snippet
        let existing_request = CreateSnippetRequest {
            trigger: "::hello".to_string(),
            replacement: "Original Hello".to_string(),
            tags: None,
        };
        create_use_case.execute(existing_request).await.unwrap();

        // Try to import snippets including the existing one
        let snippets = vec![
            ImportSnippetData {
                trigger: "::hello".to_string(),
                replacement: "New Hello".to_string(),
                tags: None,
            },
            ImportSnippetData {
                trigger: "::new".to_string(),
                replacement: "New snippet".to_string(),
                tags: None,
            },
        ];

        let request = ImportSnippetsRequest {
            snippets,
            overwrite_existing: false,
        };

        let result = import_use_case.execute(request).await.unwrap();

        assert_eq!(result.imported_count, 1); // Only the new one
        assert_eq!(result.skipped_count, 1); // The existing one was skipped
        assert_eq!(result.error_count, 0);
    }

    #[tokio::test]
    async fn test_import_with_overwrite() {
        let (import_use_case, create_use_case, _temp_dir) = create_test_use_case().await;

        // Create an existing snippet
        let existing_request = CreateSnippetRequest {
            trigger: "::hello".to_string(),
            replacement: "Original Hello".to_string(),
            tags: None,
        };
        create_use_case.execute(existing_request).await.unwrap();

        // Import with overwrite enabled
        let snippets = vec![ImportSnippetData {
            trigger: "::hello".to_string(),
            replacement: "Updated Hello".to_string(),
            tags: Some(vec!["updated".to_string()]),
        }];

        let request = ImportSnippetsRequest {
            snippets,
            overwrite_existing: true,
        };

        let result = import_use_case.execute(request).await.unwrap();

        assert_eq!(result.imported_count, 1);
        assert_eq!(result.skipped_count, 0);
        assert_eq!(result.error_count, 0);
    }

    #[tokio::test]
    async fn test_import_from_json() {
        let (import_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;

        let json_data = r#"
        [
            {
                "trigger": "::json1",
                "replacement": "JSON snippet 1",
                "tags": ["json", "test"]
            },
            {
                "trigger": "::json2",
                "replacement": "JSON snippet 2",
                "tags": null
            }
        ]
        "#;

        let result = import_use_case
            .import_from_json(json_data, false)
            .await
            .unwrap();

        assert_eq!(result.imported_count, 2);
        assert_eq!(result.error_count, 0);
    }

    #[tokio::test]
    async fn test_import_invalid_json() {
        let (import_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;

        let invalid_json = "invalid json data";
        let result = import_use_case.import_from_json(invalid_json, false).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse JSON"));
    }

    #[tokio::test]
    async fn test_import_invalid_snippet() {
        let (import_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;

        let snippets = vec![ImportSnippetData {
            trigger: "".to_string(), // Invalid empty trigger
            replacement: "Test".to_string(),
            tags: None,
        }];

        let request = ImportSnippetsRequest {
            snippets,
            overwrite_existing: false,
        };

        let result = import_use_case.execute(request).await.unwrap();

        assert_eq!(result.imported_count, 0);
        assert_eq!(result.error_count, 1);
        assert!(!result.errors.is_empty());
    }
}
