use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnippetRequest {
    pub trigger: String,
    pub replacement: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSnippetRequest {
    pub id: Uuid,
    pub trigger: Option<String>,
    pub replacement: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetDto {
    pub id: Uuid,
    pub trigger: String,
    pub replacement: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub usage_count: u64,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetListRequest {
    pub search_term: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetListResponse {
    pub snippets: Vec<SnippetDto>,
    pub total_count: u64,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub total_pages: u32,
    pub current_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionRequest {
    pub trigger: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionResponse {
    pub success: bool,
    pub expanded_text: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSnippetsRequest {
    pub snippets: Vec<ImportSnippetData>,
    pub overwrite_existing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSnippetData {
    pub trigger: String,
    pub replacement: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub imported_count: u32,
    pub skipped_count: u32,
    pub error_count: u32,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSnippetsRequest {
    pub include_inactive: bool,
    pub tags_filter: Option<Vec<String>>,
}

impl From<crate::domain::Snippet> for SnippetDto {
    fn from(snippet: crate::domain::Snippet) -> Self {
        Self {
            id: snippet.id,
            trigger: snippet.trigger,
            replacement: snippet.replacement,
            created_at: snippet.created_at,
            updated_at: snippet.updated_at,
            is_active: snippet.is_active,
            usage_count: snippet.usage_count,
            tags: snippet.tags,
        }
    }
}
