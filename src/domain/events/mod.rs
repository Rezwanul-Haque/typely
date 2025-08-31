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
        expanded_text_length: usize,
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
    pub fn snippet_created(snippet_id: Uuid, trigger: String) -> Self {
        Self::SnippetCreated {
            snippet_id,
            trigger,
            timestamp: Utc::now(),
        }
    }

    pub fn snippet_updated(snippet_id: Uuid, trigger: String) -> Self {
        Self::SnippetUpdated {
            snippet_id,
            trigger,
            timestamp: Utc::now(),
        }
    }

    pub fn snippet_deleted(snippet_id: Uuid, trigger: String) -> Self {
        Self::SnippetDeleted {
            snippet_id,
            trigger,
            timestamp: Utc::now(),
        }
    }

    pub fn snippet_expanded(snippet_id: Uuid, trigger: String, expanded_text_length: usize) -> Self {
        Self::SnippetExpanded {
            snippet_id,
            trigger,
            expanded_text_length,
            timestamp: Utc::now(),
        }
    }

    pub fn snippet_activated(snippet_id: Uuid, trigger: String) -> Self {
        Self::SnippetActivated {
            snippet_id,
            trigger,
            timestamp: Utc::now(),
        }
    }

    pub fn snippet_deactivated(snippet_id: Uuid, trigger: String) -> Self {
        Self::SnippetDeactivated {
            snippet_id,
            trigger,
            timestamp: Utc::now(),
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Self::SnippetCreated { timestamp, .. }
            | Self::SnippetUpdated { timestamp, .. }
            | Self::SnippetDeleted { timestamp, .. }
            | Self::SnippetExpanded { timestamp, .. }
            | Self::SnippetActivated { timestamp, .. }
            | Self::SnippetDeactivated { timestamp, .. } => *timestamp,
        }
    }

    pub fn snippet_id(&self) -> Uuid {
        match self {
            Self::SnippetCreated { snippet_id, .. }
            | Self::SnippetUpdated { snippet_id, .. }
            | Self::SnippetDeleted { snippet_id, .. }
            | Self::SnippetExpanded { snippet_id, .. }
            | Self::SnippetActivated { snippet_id, .. }
            | Self::SnippetDeactivated { snippet_id, .. } => *snippet_id,
        }
    }
}