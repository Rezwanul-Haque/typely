use crate::app::dto::*;
use crate::app::services::*;
use crate::domain::repositories::SnippetRepository;
use crate::infra::{DatabaseConnection, SqliteSnippetRepository};
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

/// Main application service that coordinates all services
pub struct TypelyService {
    pub create_snippet: CreateSnippetService,
    pub update_snippet: UpdateSnippetService,
    pub delete_snippet: DeleteSnippetService,
    pub get_snippet: GetSnippetService,
    pub list_snippets: ListSnippetsService,
    pub expand_snippet: ExpandSnippetService,
    pub import_snippets: ImportSnippetsService,
    pub export_snippets: ExportSnippetsService,
}

impl TypelyService {
    pub async fn new(database_connection: DatabaseConnection) -> Self {
        let repository: Arc<dyn SnippetRepository> = Arc::new(SqliteSnippetRepository::new(
            database_connection.pool().clone(),
        ));

        Self {
            create_snippet: CreateSnippetService::new(repository.clone()),
            update_snippet: UpdateSnippetService::new(repository.clone()),
            delete_snippet: DeleteSnippetService::new(repository.clone()),
            get_snippet: GetSnippetService::new(repository.clone()),
            list_snippets: ListSnippetsService::new(repository.clone()),
            expand_snippet: ExpandSnippetService::new(repository.clone()),
            import_snippets: ImportSnippetsService::new(repository.clone()),
            export_snippets: ExportSnippetsService::new(repository.clone()),
        }
    }

    // Convenience methods that delegate to use cases
    pub async fn create_snippet(&self, request: CreateSnippetRequest) -> Result<SnippetDto> {
        self.create_snippet.execute(request).await
    }

    pub async fn update_snippet(&self, request: UpdateSnippetRequest) -> Result<SnippetDto> {
        self.update_snippet.execute(request).await
    }

    pub async fn delete_snippet(&self, id: Uuid) -> Result<bool> {
        self.delete_snippet.execute(id).await
    }

    pub async fn get_snippet(&self, id: Uuid) -> Result<Option<SnippetDto>> {
        self.get_snippet.execute(id).await
    }

    pub async fn get_snippet_by_trigger(&self, trigger: &str) -> Result<Option<SnippetDto>> {
        self.get_snippet.execute_by_trigger(trigger).await
    }

    pub async fn list_snippets(&self, request: SnippetListRequest) -> Result<SnippetListResponse> {
        self.list_snippets.execute(request).await
    }

    pub async fn expand_snippet(&self, request: ExpansionRequest) -> Result<ExpansionResponse> {
        self.expand_snippet.execute(request).await
    }

    pub async fn find_matching_snippets(&self, text: &str) -> Result<Vec<String>> {
        self.expand_snippet.find_matching_snippets(text).await
    }

    pub async fn import_snippets(&self, request: ImportSnippetsRequest) -> Result<ImportResult> {
        self.import_snippets.execute(request).await
    }

    pub async fn import_from_json(&self, json_data: &str, overwrite: bool) -> Result<ImportResult> {
        self.import_snippets
            .import_from_json(json_data, overwrite)
            .await
    }

    pub async fn export_snippets(
        &self,
        request: ExportSnippetsRequest,
    ) -> Result<Vec<ImportSnippetData>> {
        self.export_snippets.execute(request).await
    }

    pub async fn export_to_json(&self, request: ExportSnippetsRequest) -> Result<String> {
        self.export_snippets.export_to_json(request).await
    }

    pub async fn export_all_to_json(&self) -> Result<String> {
        self.export_snippets.export_all_to_json().await
    }

    // Additional convenience methods
    pub async fn get_all_active_snippets(&self) -> Result<Vec<SnippetDto>> {
        let request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: Some(true),
            limit: None,
            offset: None,
            sort_by: Some("updated".to_string()),
            sort_order: Some("desc".to_string()),
        };

        let response = self.list_snippets(request).await?;
        Ok(response.snippets)
    }

    pub async fn search_snippets(&self, search_term: &str) -> Result<Vec<SnippetDto>> {
        let request = SnippetListRequest {
            search_term: Some(search_term.to_string()),
            tags: None,
            is_active: Some(true),
            limit: Some(50),
            offset: None,
            sort_by: Some("usage".to_string()),
            sort_order: Some("desc".to_string()),
        };

        let response = self.list_snippets(request).await?;
        Ok(response.snippets)
    }

    pub async fn get_snippets_by_tag(&self, tag: &str) -> Result<Vec<SnippetDto>> {
        let request = SnippetListRequest {
            search_term: None,
            tags: Some(vec![tag.to_string()]),
            is_active: Some(true),
            limit: None,
            offset: None,
            sort_by: Some("updated".to_string()),
            sort_order: Some("desc".to_string()),
        };

        let response = self.list_snippets(request).await?;
        Ok(response.snippets)
    }

    pub async fn get_most_used_snippets(&self, limit: u32) -> Result<Vec<SnippetDto>> {
        let request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: Some(true),
            limit: Some(limit),
            offset: None,
            sort_by: Some("usage".to_string()),
            sort_order: Some("desc".to_string()),
        };

        let response = self.list_snippets(request).await?;
        Ok(response.snippets)
    }

    pub async fn get_recent_snippets(&self, limit: u32) -> Result<Vec<SnippetDto>> {
        let request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: Some(true),
            limit: Some(limit),
            offset: None,
            sort_by: Some("updated".to_string()),
            sort_order: Some("desc".to_string()),
        };

        let response = self.list_snippets(request).await?;
        Ok(response.snippets)
    }

    pub async fn activate_snippet(&self, id: Uuid) -> Result<SnippetDto> {
        let request = UpdateSnippetRequest {
            id,
            trigger: None,
            replacement: None,
            tags: None,
            is_active: Some(true),
        };

        self.update_snippet(request).await
    }

    pub async fn deactivate_snippet(&self, id: Uuid) -> Result<SnippetDto> {
        let request = UpdateSnippetRequest {
            id,
            trigger: None,
            replacement: None,
            tags: None,
            is_active: Some(false),
        };

        self.update_snippet(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::DatabaseConnection;
    use tempfile::TempDir;

    async fn create_test_service() -> (TypelyService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let service = TypelyService::new(db_connection).await;
        (service, temp_dir)
    }

    #[tokio::test]
    async fn test_service_integration() {
        let (service, _temp_dir) = create_test_service().await;

        // Create a snippet
        let create_request = CreateSnippetRequest {
            trigger: "::hello".to_string(),
            replacement: "Hello, World!".to_string(),
            tags: Some(vec!["greeting".to_string()]),
        };

        let created = service.create_snippet(create_request).await.unwrap();
        assert_eq!(created.trigger, "::hello");

        // Get the snippet
        let found = service.get_snippet(created.id).await.unwrap();
        assert!(found.is_some());

        // List snippets
        let list_request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: None,
            limit: None,
            offset: None,
            sort_by: None,
            sort_order: None,
        };

        let list_response = service.list_snippets(list_request).await.unwrap();
        assert_eq!(list_response.snippets.len(), 1);

        // Expand snippet
        let expansion_request = ExpansionRequest {
            trigger: "::hello".to_string(),
            context: None,
        };

        let expansion_response = service.expand_snippet(expansion_request).await.unwrap();
        assert!(expansion_response.success);
        assert_eq!(expansion_response.expanded_text.unwrap(), "Hello, World!");

        // Delete snippet
        let deleted = service.delete_snippet(created.id).await.unwrap();
        assert!(deleted);
    }

    #[tokio::test]
    async fn test_convenience_methods() {
        let (service, _temp_dir) = create_test_service().await;

        // Create some test snippets
        for i in 1..=3 {
            let request = CreateSnippetRequest {
                trigger: format!("::test{}", i),
                replacement: format!("Test {}", i),
                tags: Some(vec!["test".to_string()]),
            };
            service.create_snippet(request).await.unwrap();
        }

        // Test get all active snippets
        let active_snippets = service.get_all_active_snippets().await.unwrap();
        assert_eq!(active_snippets.len(), 3);

        // Test search snippets
        let search_results = service.search_snippets("Test").await.unwrap();
        assert_eq!(search_results.len(), 3);

        // Test get snippets by tag
        let tagged_snippets = service.get_snippets_by_tag("test").await.unwrap();
        assert_eq!(tagged_snippets.len(), 3);
    }
}
