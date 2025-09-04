use serde::{Deserialize, Serialize};

/// Simple trigger detection service for CLI builds (stub implementation)
#[derive(Clone)]
pub struct TriggerDetectionService;

impl Default for TriggerDetectionService {
    fn default() -> Self {
        Self::new()
    }
}

impl TriggerDetectionService {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_trigger(&self, text: &str) -> Option<TriggerMatch> {
        // Simple detection - look for :: prefix
        if let Some(start) = text.rfind("::") {
            let trigger = &text[start..];
            if trigger.len() > 2 {
                return Some(TriggerMatch {
                    trigger: trigger.to_string(),
                    start_position: start,
                    end_position: text.len(),
                });
            }
        }
        None
    }

    pub fn find_triggers_in_text(&self, text: &str) -> Vec<TriggerMatch> {
        let mut matches = Vec::new();
        let mut current_pos = 0;

        while let Some(pos) = text[current_pos..].find("::") {
            let absolute_pos = current_pos + pos;

            // Find end of trigger (whitespace or end of string)
            let remaining = &text[absolute_pos..];
            let end_pos = remaining
                .find(char::is_whitespace)
                .map(|p| absolute_pos + p)
                .unwrap_or(text.len());

            let trigger = text[absolute_pos..end_pos].to_string();
            if trigger.len() > 2 {
                matches.push(TriggerMatch {
                    trigger,
                    start_position: absolute_pos,
                    end_position: end_pos,
                });
            }

            current_pos = end_pos;
        }

        matches
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerMatch {
    pub trigger: String,
    pub start_position: usize,
    pub end_position: usize,
}

impl TriggerMatch {
    pub fn new(trigger: String, start: usize, end: usize) -> Self {
        Self {
            trigger,
            start_position: start,
            end_position: end,
        }
    }

    pub fn length(&self) -> usize {
        self.end_position - self.start_position
    }
}
