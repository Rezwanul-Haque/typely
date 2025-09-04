use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snippet {
    pub id: Uuid,
    pub trigger: String,
    pub replacement: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub usage_count: u64,
    pub tags: Vec<String>,
}

impl Snippet {
    pub fn new(trigger: String, replacement: String) -> anyhow::Result<Self> {
        Self::validate_trigger(&trigger)?;
        Self::validate_replacement(&replacement)?;

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            trigger,
            replacement,
            created_at: now,
            updated_at: now,
            is_active: true,
            usage_count: 0,
            tags: Vec::new(),
        })
    }

    pub fn with_id(
        id: Uuid,
        trigger: String,
        replacement: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> anyhow::Result<Self> {
        Self::validate_trigger(&trigger)?;
        Self::validate_replacement(&replacement)?;

        Ok(Self {
            id,
            trigger,
            replacement,
            created_at,
            updated_at,
            is_active: true,
            usage_count: 0,
            tags: Vec::new(),
        })
    }

    pub fn update_replacement(&mut self, replacement: String) -> anyhow::Result<()> {
        Self::validate_replacement(&replacement)?;
        self.replacement = replacement;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_trigger(&mut self, trigger: String) -> anyhow::Result<()> {
        Self::validate_trigger(&trigger)?;
        self.trigger = trigger;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn increment_usage(&mut self) {
        self.usage_count = self.usage_count.saturating_add(1);
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    fn validate_trigger(trigger: &str) -> anyhow::Result<()> {
        if trigger.is_empty() {
            return Err(anyhow::anyhow!("Trigger cannot be empty"));
        }

        if trigger.len() > 50 {
            return Err(anyhow::anyhow!(
                "Trigger cannot be longer than 50 characters"
            ));
        }

        if trigger.contains(' ') {
            return Err(anyhow::anyhow!("Trigger cannot contain spaces"));
        }

        if !trigger
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || ":_-".contains(c))
        {
            return Err(anyhow::anyhow!("Trigger can only contain alphanumeric characters, colons, underscores, and hyphens"));
        }

        Ok(())
    }

    fn validate_replacement(replacement: &str) -> anyhow::Result<()> {
        if replacement.is_empty() {
            return Err(anyhow::anyhow!("Replacement cannot be empty"));
        }

        if replacement.len() > 10000 {
            return Err(anyhow::anyhow!(
                "Replacement cannot be longer than 10000 characters"
            ));
        }

        Ok(())
    }

    pub fn expand(&self) -> String {
        let mut result = self.replacement.clone();

        // Handle dynamic placeholders
        let now = Utc::now();

        // Date/time placeholders
        result = result.replace("{date}", &now.format("%Y-%m-%d").to_string());
        result = result.replace("{time}", &now.format("%H:%M:%S").to_string());
        result = result.replace("{datetime}", &now.format("%Y-%m-%d %H:%M:%S").to_string());
        result = result.replace("{timestamp}", &now.timestamp().to_string());

        // User info placeholders (simplified for now)
        if let Ok(username) = std::env::var("USER") {
            result = result.replace("{user}", &username);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snippet_creation() {
        let snippet = Snippet::new("::test".to_string(), "Test replacement".to_string()).unwrap();
        assert_eq!(snippet.trigger, "::test");
        assert_eq!(snippet.replacement, "Test replacement");
        assert_eq!(snippet.usage_count, 0);
        assert!(snippet.is_active);
    }

    #[test]
    fn test_invalid_trigger() {
        let result = Snippet::new("".to_string(), "Test".to_string());
        assert!(result.is_err());

        let result = Snippet::new("test with spaces".to_string(), "Test".to_string());
        assert!(result.is_err());

        let result = Snippet::new("test@invalid".to_string(), "Test".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_replacement() {
        let result = Snippet::new("::test".to_string(), "".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_update_operations() {
        let mut snippet = Snippet::new("::test".to_string(), "Original".to_string()).unwrap();
        let original_time = snippet.updated_at;

        std::thread::sleep(std::time::Duration::from_millis(1));
        snippet.update_replacement("Updated".to_string()).unwrap();
        assert_eq!(snippet.replacement, "Updated");
        assert!(snippet.updated_at > original_time);
    }

    #[test]
    fn test_usage_increment() {
        let mut snippet = Snippet::new("::test".to_string(), "Test".to_string()).unwrap();
        assert_eq!(snippet.usage_count, 0);

        snippet.increment_usage();
        assert_eq!(snippet.usage_count, 1);
    }

    #[test]
    fn test_tag_management() {
        let mut snippet = Snippet::new("::test".to_string(), "Test".to_string()).unwrap();

        snippet.add_tag("work".to_string());
        assert!(snippet.tags.contains(&"work".to_string()));

        snippet.add_tag("work".to_string()); // Duplicate should not be added
        assert_eq!(snippet.tags.len(), 1);

        snippet.remove_tag("work");
        assert!(!snippet.tags.contains(&"work".to_string()));
    }

    #[test]
    fn test_expansion_with_placeholders() {
        let snippet = Snippet::new("::date".to_string(), "Today is {date}".to_string()).unwrap();
        let expanded = snippet.expand();
        assert!(expanded.contains("Today is"));
        assert!(expanded.len() > "Today is ".len());
    }
}
