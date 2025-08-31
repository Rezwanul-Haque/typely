use crate::domain::TriggerMatch;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TRIGGER_PATTERNS: Vec<Regex> = vec![
        // Standard :: pattern
        Regex::new(r"::[a-zA-Z0-9_-]+").unwrap(),
        // Alternative patterns for different trigger styles
        Regex::new(r"@[a-zA-Z0-9_-]+").unwrap(),
        Regex::new(r"#[a-zA-Z0-9_-]+").unwrap(),
    ];
}

#[derive(Clone)]
pub struct TriggerDetectionService {
    enabled_patterns: Vec<usize>,
}

impl TriggerDetectionService {
    pub fn new() -> Self {
        Self {
            enabled_patterns: vec![0], // Only enable :: pattern by default
        }
    }

    pub fn with_all_patterns() -> Self {
        Self {
            enabled_patterns: (0..TRIGGER_PATTERNS.len()).collect(),
        }
    }

    pub fn enable_pattern(&mut self, pattern_index: usize) {
        if pattern_index < TRIGGER_PATTERNS.len() && !self.enabled_patterns.contains(&pattern_index) {
            self.enabled_patterns.push(pattern_index);
        }
    }

    pub fn disable_pattern(&mut self, pattern_index: usize) {
        self.enabled_patterns.retain(|&x| x != pattern_index);
    }

    pub fn find_triggers_in_text(&self, text: &str) -> Vec<TriggerMatch> {
        let mut matches = Vec::new();
        
        for &pattern_index in &self.enabled_patterns {
            if let Some(regex) = TRIGGER_PATTERNS.get(pattern_index) {
                for m in regex.find_iter(text) {
                    matches.push(TriggerMatch::new(
                        m.as_str().to_string(),
                        m.start(),
                        m.end(),
                    ));
                }
            }
        }
        
        // Sort by position
        matches.sort_by_key(|m| m.start_position);
        matches
    }

    pub fn find_trigger_at_cursor(&self, text: &str, cursor_position: usize) -> Option<TriggerMatch> {
        let matches = self.find_triggers_in_text(text);
        
        // Find the trigger that contains or is immediately before the cursor
        matches
            .into_iter()
            .find(|m| cursor_position >= m.start_position && cursor_position <= m.end_position)
    }

    pub fn extract_partial_trigger(&self, text: &str, cursor_position: usize) -> Option<String> {
        if cursor_position == 0 || cursor_position > text.len() {
            return None;
        }

        // Look backwards from cursor to find potential trigger start
        let chars: Vec<char> = text.chars().collect();
        
        // Find the start of a potential trigger
        let mut start = cursor_position.saturating_sub(1);
        while start > 0 {
            if chars[start] == ':' && start > 0 && chars[start - 1] == ':' {
                start -= 1;
                break;
            }
            if !chars[start].is_ascii_alphanumeric() && chars[start] != '_' && chars[start] != '-' {
                return None;
            }
            start -= 1;
        }

        // Extract the partial trigger
        let partial: String = chars[start..cursor_position].iter().collect();
        
        // Validate it looks like a trigger
        if partial.starts_with("::") && partial.len() > 2 {
            Some(partial)
        } else {
            None
        }
    }

    pub fn is_valid_trigger_character(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == ':'
    }
}

impl Default for TriggerDetectionService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_triggers_standard_pattern() {
        let service = TriggerDetectionService::new();
        let text = "Hello ::world and ::test!";
        let triggers = service.find_triggers_in_text(text);
        
        assert_eq!(triggers.len(), 2);
        assert_eq!(triggers[0].trigger, "::world");
        assert_eq!(triggers[1].trigger, "::test");
    }

    #[test]
    fn test_find_trigger_at_cursor() {
        let service = TriggerDetectionService::new();
        let text = "Hello ::world";
        
        // Cursor at end of trigger
        let trigger = service.find_trigger_at_cursor(text, 13);
        assert!(trigger.is_some());
        assert_eq!(trigger.unwrap().trigger, "::world");
        
        // Cursor not in trigger
        let trigger = service.find_trigger_at_cursor(text, 5);
        assert!(trigger.is_none());
    }

    #[test]
    fn test_extract_partial_trigger() {
        let service = TriggerDetectionService::new();
        
        let partial = service.extract_partial_trigger("Hello ::wor", 11);
        assert_eq!(partial, Some("::wor".to_string()));
        
        let partial = service.extract_partial_trigger("Hello ::", 8);
        assert_eq!(partial, Some("::".to_string()));
        
        let partial = service.extract_partial_trigger("Hello world", 11);
        assert_eq!(partial, None);
    }

    #[test]
    fn test_multiple_patterns() {
        let mut service = TriggerDetectionService::with_all_patterns();
        let text = "Test ::colon @at and #hash triggers";
        let triggers = service.find_triggers_in_text(text);
        
        assert_eq!(triggers.len(), 3);
        assert_eq!(triggers[0].trigger, "::colon");
        assert_eq!(triggers[1].trigger, "@at");
        assert_eq!(triggers[2].trigger, "#hash");
    }
}