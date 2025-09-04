use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainEvent {
    SnippetCreated {
        snippet_id: Uuid,
        trigger: String,
        timestamp: DateTime<Utc>,
    },
    SnippetUpdated {
        snippet_id: Uuid,
        trigger: String,
        timestamp: DateTime<Utc>,
    },
    SnippetDeleted {
        snippet_id: Uuid,
        trigger: String,
        timestamp: DateTime<Utc>,
    },
    SnippetExpanded {
        snippet_id: Uuid,
        trigger: String,
        timestamp: DateTime<Utc>,
    },
    SnippetActivated {
        snippet_id: Uuid,
        trigger: String,
        timestamp: DateTime<Utc>,
    },
    SnippetDeactivated {
        snippet_id: Uuid,
        trigger: String,
        timestamp: DateTime<Utc>,
    },
}

impl DomainEvent {
    pub fn snippet_id(&self) -> Uuid {
        match self {
            DomainEvent::SnippetCreated { snippet_id, .. } => *snippet_id,
            DomainEvent::SnippetUpdated { snippet_id, .. } => *snippet_id,
            DomainEvent::SnippetDeleted { snippet_id, .. } => *snippet_id,
            DomainEvent::SnippetExpanded { snippet_id, .. } => *snippet_id,
            DomainEvent::SnippetActivated { snippet_id, .. } => *snippet_id,
            DomainEvent::SnippetDeactivated { snippet_id, .. } => *snippet_id,
        }
    }

    pub fn trigger(&self) -> &str {
        match self {
            DomainEvent::SnippetCreated { trigger, .. } => trigger,
            DomainEvent::SnippetUpdated { trigger, .. } => trigger,
            DomainEvent::SnippetDeleted { trigger, .. } => trigger,
            DomainEvent::SnippetExpanded { trigger, .. } => trigger,
            DomainEvent::SnippetActivated { trigger, .. } => trigger,
            DomainEvent::SnippetDeactivated { trigger, .. } => trigger,
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            DomainEvent::SnippetCreated { timestamp, .. } => *timestamp,
            DomainEvent::SnippetUpdated { timestamp, .. } => *timestamp,
            DomainEvent::SnippetDeleted { timestamp, .. } => *timestamp,
            DomainEvent::SnippetExpanded { timestamp, .. } => *timestamp,
            DomainEvent::SnippetActivated { timestamp, .. } => *timestamp,
            DomainEvent::SnippetDeactivated { timestamp, .. } => *timestamp,
        }
    }
}
