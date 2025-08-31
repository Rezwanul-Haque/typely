use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{Snippet, SnippetQuery};

#[async_trait]
pub trait SnippetRepository: Send + Sync {
    async fn save(&self, snippet: &Snippet) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<Snippet>>;
    async fn find_by_trigger(&self, trigger: &str) -> anyhow::Result<Option<Snippet>>;
    async fn find_all(&self, query: &SnippetQuery) -> anyhow::Result<Vec<Snippet>>;
    async fn update(&self, snippet: &Snippet) -> anyhow::Result<()>;
    async fn delete(&self, id: &Uuid) -> anyhow::Result<bool>;
    async fn count(&self, query: &SnippetQuery) -> anyhow::Result<u64>;
    async fn exists_with_trigger(&self, trigger: &str) -> anyhow::Result<bool>;
}