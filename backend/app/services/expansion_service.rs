use crate::domain::{Snippet, ExpansionResult, TriggerMatch};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TRIGGER_REGEX: Regex = Regex::new(r"::[a-zA-Z0-9_-]+").unwrap();
}

pub struct ExpansionService;

impl ExpansionService {
    pub fn new() -> Self {
        Self
    }

    pub fn find_triggers(&self, text: &str) -> Vec<TriggerMatch> {
        TRIGGER_REGEX
            .find_iter(text)
            .map(|m| TriggerMatch::new(m.as_str().to_string(), m.start(), m.end()))
            .collect()
    }

    pub fn expand_snippet(&self, snippet: &Snippet, context: &ExpansionContext) -> ExpansionResult {
        match self.try_expand(snippet, context) {
            Ok(expanded) => ExpansionResult::success(
                snippet.id,
                snippet.trigger.clone(),
                snippet.replacement.clone(),
                expanded,
            ),
            Err(e) => ExpansionResult::failure(
                snippet.id,
                snippet.trigger.clone(),
                snippet.replacement.clone(),
                e.to_string(),
            ),
        }
    }

    fn try_expand(&self, snippet: &Snippet, _context: &ExpansionContext) -> anyhow::Result<String> {
        // For now, just use the snippet's built-in expansion
        // In the future, we could add more sophisticated expansion logic here
        Ok(snippet.expand())
    }

    pub fn replace_in_text(&self, text: &str, snippets: &HashMap<String, &Snippet>) -> String {
        let mut result = text.to_string();
        
        // Find all trigger matches in the text
        let matches = self.find_triggers(text);
        
        // Sort matches by position (descending) to avoid position shifts during replacement
        let mut sorted_matches = matches;
        sorted_matches.sort_by(|a, b| b.start_position.cmp(&a.start_position));
        
        for trigger_match in sorted_matches {
            if let Some(snippet) = snippets.get(&trigger_match.trigger) {
                if snippet.is_active {
                    let expanded = snippet.expand();
                    result.replace_range(
                        trigger_match.start_position..trigger_match.end_position,
                        &expanded,
                    );
                }
            }
        }
        
        result
    }
}

#[derive(Debug, Clone)]
pub struct ExpansionContext {
    pub cursor_position: Option<usize>,
    pub surrounding_text: Option<String>,
    pub application_context: Option<String>,
}

impl Default for ExpansionContext {
    fn default() -> Self {
        Self {
            cursor_position: None,
            surrounding_text: None,
            application_context: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Snippet;
    use uuid::Uuid;

    #[test]
    fn test_find_triggers() {
        let service = ExpansionService::new();
        let text = "Hello ::world and ::test";
        let triggers = service.find_triggers(text);
        
        assert_eq!(triggers.len(), 2);
        assert_eq!(triggers[0].trigger, "::world");
        assert_eq!(triggers[1].trigger, "::test");
    }

    #[test]
    fn test_replace_in_text() {
        let service = ExpansionService::new();
        let snippet1 = Snippet::new("::hello".to_string(), "Hello, World!".to_string()).unwrap();
        let snippet2 = Snippet::new("::test".to_string(), "This is a test".to_string()).unwrap();
        
        let mut snippets = HashMap::new();
        snippets.insert("::hello".to_string(), &snippet1);
        snippets.insert("::test".to_string(), &snippet2);
        
        let text = "Say ::hello and ::test";
        let result = service.replace_in_text(text, &snippets);
        
        assert_eq!(result, "Say Hello, World! and This is a test");
    }

    #[test]
    fn test_no_replacement_for_inactive_snippets() {
        let service = ExpansionService::new();
        let mut snippet = Snippet::new("::test".to_string(), "Replacement".to_string()).unwrap();
        snippet.deactivate();
        
        let mut snippets = HashMap::new();
        snippets.insert("::test".to_string(), &snippet);
        
        let text = "This is ::test";
        let result = service.replace_in_text(text, &snippets);
        
        assert_eq!(result, "This is ::test"); // Should remain unchanged
    }
}