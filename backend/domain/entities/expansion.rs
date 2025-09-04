use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpansionContext {
    pub cursor_position: Option<usize>,
    pub surrounding_text: Option<String>,
    pub application_context: Option<String>,
}

impl ExpansionContext {
    pub fn new() -> Self {
        Self {
            cursor_position: None,
            surrounding_text: None,
            application_context: None,
        }
    }

    pub fn with_cursor_position(mut self, position: usize) -> Self {
        self.cursor_position = Some(position);
        self
    }

    pub fn with_surrounding_text(mut self, text: String) -> Self {
        self.surrounding_text = Some(text);
        self
    }

    pub fn with_application_context(mut self, context: String) -> Self {
        self.application_context = Some(context);
        self
    }
}

impl Default for ExpansionContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpansionResult {
    pub original_text: String,
    pub expanded_text: String,
    pub success: bool,
    pub error: Option<String>,
}

impl ExpansionResult {
    pub fn success(original: String, expanded: String) -> Self {
        Self {
            original_text: original,
            expanded_text: expanded,
            success: true,
            error: None,
        }
    }

    pub fn failure(original: String, error: String) -> Self {
        Self {
            original_text: original,
            expanded_text: String::new(),
            success: false,
            error: Some(error),
        }
    }
}

/// Simple expansion service for CLI-only builds
pub struct ExpansionService;

impl ExpansionService {
    pub fn new() -> Self {
        Self
    }

    /// Expand text with placeholders
    pub fn expand_text(&self, text: &str, _context: &ExpansionContext) -> ExpansionResult {
        let expanded = self.process_placeholders(text);
        ExpansionResult::success(text.to_string(), expanded)
    }

    /// Expand a snippet (used by expand_snippet service)
    pub fn expand_snippet(
        &self,
        snippet: &crate::domain::Snippet,
        _context: &ExpansionContext,
    ) -> ExpansionResult {
        let expanded = self.process_placeholders(&snippet.replacement);
        ExpansionResult::success(snippet.replacement.clone(), expanded)
    }

    /// Find triggers in text (stub implementation)
    pub fn find_triggers(&self, text: &str) -> Vec<super::triggers::TriggerMatch> {
        let mut matches = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut pos = 0;

        for word in words {
            if let Some(word_pos) = text[pos..].find(word) {
                let absolute_pos = pos + word_pos;
                if word.starts_with("::") && word.len() > 2 {
                    matches.push(super::triggers::TriggerMatch::new(
                        word.to_string(),
                        absolute_pos,
                        absolute_pos + word.len(),
                    ));
                }
                pos = absolute_pos + word.len();
            }
        }
        matches
    }

    fn process_placeholders(&self, text: &str) -> String {
        let mut result = text.to_string();
        let now = chrono::Utc::now();

        // Date/time placeholders
        result = result.replace("{date}", &now.format("%Y-%m-%d").to_string());
        result = result.replace("{time}", &now.format("%H:%M:%S").to_string());
        result = result.replace("{datetime}", &now.format("%Y-%m-%d %H:%M:%S").to_string());
        result = result.replace("{timestamp}", &now.timestamp().to_string());

        // User info placeholders (simplified for CLI)
        if let Ok(username) = std::env::var("USER") {
            result = result.replace("{user}", &username);
        }

        result
    }
}

impl Default for ExpansionService {
    fn default() -> Self {
        Self::new()
    }
}
