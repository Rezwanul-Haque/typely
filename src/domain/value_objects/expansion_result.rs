use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpansionResult {
    pub snippet_id: Uuid,
    pub trigger: String,
    pub original_text: String,
    pub expanded_text: String,
    pub success: bool,
    pub error_message: Option<String>,
}

impl ExpansionResult {
    pub fn success(snippet_id: Uuid, trigger: String, original_text: String, expanded_text: String) -> Self {
        Self {
            snippet_id,
            trigger,
            original_text,
            expanded_text,
            success: true,
            error_message: None,
        }
    }

    pub fn failure(snippet_id: Uuid, trigger: String, original_text: String, error: String) -> Self {
        Self {
            snippet_id,
            trigger,
            original_text: original_text.clone(),
            expanded_text: original_text,
            success: false,
            error_message: Some(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerMatch {
    pub trigger: String,
    pub start_position: usize,
    pub end_position: usize,
    pub confidence: f32,
}

impl TriggerMatch {
    pub fn new(trigger: String, start_position: usize, end_position: usize) -> Self {
        Self {
            trigger,
            start_position,
            end_position,
            confidence: 1.0,
        }
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn length(&self) -> usize {
        self.end_position - self.start_position
    }
}