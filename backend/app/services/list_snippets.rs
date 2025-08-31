use crate::app::dto::{SnippetListRequest, SnippetListResponse, SnippetDto, PageInfo};
use crate::domain::{SnippetRepository, SnippetQuery, SortBy, SortOrder};
use anyhow::Result;
use std::sync::Arc;

pub struct ListSnippetsService {
    repository: Arc<dyn SnippetRepository>,
}

impl ListSnippetsService {
    pub fn new(repository: Arc<dyn SnippetRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, request: SnippetListRequest) -> Result<SnippetListResponse> {
        // Convert request to domain query
        let query = self.convert_request_to_query(&request);
        
        // Get snippets and total count
        let snippets = self.repository.find_all(&query).await?;
        let total_count = self.repository.count(&query).await?;
        
        // Convert domain snippets to DTOs
        let snippet_dtos: Vec<SnippetDto> = snippets
            .into_iter()
            .map(SnippetDto::from)
            .collect();
        
        // Calculate pagination info
        let page_info = self.calculate_page_info(&request, total_count);
        
        Ok(SnippetListResponse {
            snippets: snippet_dtos,
            total_count,
            page_info,
        })
    }

    fn convert_request_to_query(&self, request: &SnippetListRequest) -> SnippetQuery {
        let mut query = SnippetQuery::new();
        
        // Apply filters
        if let Some(ref search_term) = request.search_term {
            query = query.with_search(search_term.clone());
        }
        
        if let Some(ref tags) = request.tags {
            query = query.with_tags(tags.clone());
        }
        
        if let Some(is_active) = request.is_active {
            if is_active {
                query = query.with_active_only();
            } else {
                query = query.with_all();
            }
        }
        
        // Apply pagination
        if let Some(limit) = request.limit {
            query = query.with_limit(limit);
        }
        
        if let Some(offset) = request.offset {
            query = query.with_offset(offset);
        }
        
        // Apply sorting
        if let Some(ref sort_by) = request.sort_by {
            match sort_by.as_str() {
                "usage" => query = query.sort_by_usage(),
                "created" => query = query.sort_by_created(),
                "alphabetical" => query = query.sort_alphabetically(),
                _ => {} // Use default sorting
            }
        }
        
        // Override sort order if specified
        if let Some(ref sort_order) = request.sort_order {
            match sort_order.as_str() {
                "asc" => query.sort_order = Some(SortOrder::Asc),
                "desc" => query.sort_order = Some(SortOrder::Desc),
                _ => {} // Keep current sort order
            }
        }
        
        query
    }

    fn calculate_page_info(&self, request: &SnippetListRequest, total_count: u64) -> PageInfo {
        let limit = request.limit.unwrap_or(100) as u64;
        let offset = request.offset.unwrap_or(0) as u64;
        
        let current_page = if limit > 0 { (offset / limit) + 1 } else { 1 };
        let total_pages = if limit > 0 { 
            ((total_count + limit - 1) / limit) as u32 
        } else { 
            1 
        };
        
        let has_previous_page = offset > 0;
        let has_next_page = offset + limit < total_count;
        
        PageInfo {
            has_next_page,
            has_previous_page,
            total_pages,
            current_page: current_page as u32,
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

    async fn create_test_use_case() -> (ListSnippetsService, CreateSnippetService, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let repository = Arc::new(SqliteSnippetRepository::new(db_connection.pool().clone()));
        let list_use_case = ListSnippetsService::new(repository.clone());
        let create_use_case = CreateSnippetService::new(repository);
        (list_use_case, create_use_case, temp_dir)
    }

    #[tokio::test]
    async fn test_list_empty_snippets() {
        let (list_use_case, _create_use_case, _temp_dir) = create_test_use_case().await;
        
        let request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: None,
            limit: None,
            offset: None,
            sort_by: None,
            sort_order: None,
        };
        
        let response = list_use_case.execute(request).await.unwrap();
        
        assert_eq!(response.snippets.len(), 0);
        assert_eq!(response.total_count, 0);
        assert!(!response.page_info.has_next_page);
        assert!(!response.page_info.has_previous_page);
    }

    #[tokio::test]
    async fn test_list_snippets_with_data() {
        let (list_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create some test snippets
        for i in 1..=3 {
            let request = CreateSnippetRequest {
                trigger: format!("::test{}", i),
                replacement: format!("Test {}", i),
                tags: Some(vec!["test".to_string()]),
            };
            create_use_case.execute(request).await.unwrap();
        }
        
        let request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: Some(true),
            limit: None,
            offset: None,
            sort_by: None,
            sort_order: None,
        };
        
        let response = list_use_case.execute(request).await.unwrap();
        
        assert_eq!(response.snippets.len(), 3);
        assert_eq!(response.total_count, 3);
    }

    #[tokio::test]
    async fn test_list_snippets_with_search() {
        let (list_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create test snippets
        let request1 = CreateSnippetRequest {
            trigger: "::hello".to_string(),
            replacement: "Hello, World!".to_string(),
            tags: None,
        };
        let request2 = CreateSnippetRequest {
            trigger: "::test".to_string(),
            replacement: "Test replacement".to_string(),
            tags: None,
        };
        
        create_use_case.execute(request1).await.unwrap();
        create_use_case.execute(request2).await.unwrap();
        
        // Search for "hello"
        let search_request = SnippetListRequest {
            search_term: Some("hello".to_string()),
            tags: None,
            is_active: None,
            limit: None,
            offset: None,
            sort_by: None,
            sort_order: None,
        };
        
        let response = list_use_case.execute(search_request).await.unwrap();
        
        assert_eq!(response.snippets.len(), 1);
        assert_eq!(response.snippets[0].trigger, "::hello");
    }

    #[tokio::test]
    async fn test_list_snippets_with_pagination() {
        let (list_use_case, create_use_case, _temp_dir) = create_test_use_case().await;
        
        // Create 5 test snippets
        for i in 1..=5 {
            let request = CreateSnippetRequest {
                trigger: format!("::test{}", i),
                replacement: format!("Test {}", i),
                tags: None,
            };
            create_use_case.execute(request).await.unwrap();
        }
        
        // Get first page (2 items)
        let request = SnippetListRequest {
            search_term: None,
            tags: None,
            is_active: None,
            limit: Some(2),
            offset: Some(0),
            sort_by: None,
            sort_order: None,
        };
        
        let response = list_use_case.execute(request).await.unwrap();
        
        assert_eq!(response.snippets.len(), 2);
        assert_eq!(response.total_count, 5);
        assert!(response.page_info.has_next_page);
        assert!(!response.page_info.has_previous_page);
        assert_eq!(response.page_info.current_page, 1);
        assert_eq!(response.page_info.total_pages, 3);
    }
}