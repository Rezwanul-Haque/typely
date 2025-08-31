use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnippetQuery {
    pub search: Option<String>,
    pub tags: Vec<String>,
    pub is_active: Option<bool>,
    pub sort_by: Option<SortBy>,
    pub sort_order: Option<SortOrder>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Default for SnippetQuery {
    fn default() -> Self {
        Self {
            search: None,
            tags: Vec::new(),
            is_active: None,
            sort_by: Some(SortBy::UpdatedAt),
            sort_order: Some(SortOrder::Desc),
            limit: None,
            offset: None,
        }
    }
}

impl SnippetQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_search(mut self, search: String) -> Self {
        self.search = Some(search);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn with_sort(mut self, sort_by: SortBy, sort_order: SortOrder) -> Self {
        self.sort_by = Some(sort_by);
        self.sort_order = Some(sort_order);
        self
    }

    pub fn with_pagination(mut self, limit: u32, offset: u32) -> Self {
        self.limit = Some(limit);
        self.offset = Some(offset);
        self
    }

    pub fn with_active_only(mut self) -> Self {
        self.is_active = Some(true);
        self
    }

    pub fn with_all(self) -> Self {
        // This keeps the query as is, showing all snippets regardless of active status
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn sort_by_usage(mut self) -> Self {
        self.sort_by = Some(SortBy::UsageCount);
        self
    }

    pub fn sort_by_created(mut self) -> Self {
        self.sort_by = Some(SortBy::CreatedAt);
        self
    }

    pub fn sort_alphabetically(mut self) -> Self {
        self.sort_by = Some(SortBy::Trigger);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortBy {
    Trigger,
    CreatedAt,
    UpdatedAt,
    UsageCount,
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::UpdatedAt
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Desc
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnippetStats {
    pub total_snippets: u64,
    pub active_snippets: u64,
    pub inactive_snippets: u64,
    pub total_usage_count: u64,
    pub most_used_snippet_id: Option<Uuid>,
}

impl SnippetStats {
    pub fn new(
        total_snippets: u64,
        active_snippets: u64,
        inactive_snippets: u64,
        total_usage_count: u64,
        most_used_snippet_id: Option<Uuid>,
    ) -> Self {
        Self {
            total_snippets,
            active_snippets,
            inactive_snippets,
            total_usage_count,
            most_used_snippet_id,
        }
    }
}