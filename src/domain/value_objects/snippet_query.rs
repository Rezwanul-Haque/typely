use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnippetQuery {
    pub search_term: Option<String>,
    pub tags: Vec<String>,
    pub is_active: Option<bool>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortBy {
    CreatedAt,
    UpdatedAt,
    UsageCount,
    Trigger,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SnippetQuery {
    fn default() -> Self {
        Self {
            search_term: None,
            tags: Vec::new(),
            is_active: Some(true),
            limit: Some(100),
            offset: None,
            sort_by: SortBy::UpdatedAt,
            sort_order: SortOrder::Descending,
        }
    }
}

impl SnippetQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_search_term(mut self, term: String) -> Self {
        self.search_term = Some(term);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_active_only(mut self) -> Self {
        self.is_active = Some(true);
        self
    }

    pub fn with_all(mut self) -> Self {
        self.is_active = None;
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
        self.sort_by = SortBy::UsageCount;
        self.sort_order = SortOrder::Descending;
        self
    }

    pub fn sort_by_created(mut self) -> Self {
        self.sort_by = SortBy::CreatedAt;
        self.sort_order = SortOrder::Descending;
        self
    }

    pub fn sort_alphabetically(mut self) -> Self {
        self.sort_by = SortBy::Trigger;
        self.sort_order = SortOrder::Ascending;
        self
    }
}