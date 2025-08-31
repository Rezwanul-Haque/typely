use clipboard::{ClipboardContext, ClipboardProvider};
use anyhow::Result;

pub struct ClipboardManager {
    ctx: ClipboardContext,
}

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        let ctx = ClipboardProvider::new()
            .map_err(|e| anyhow::anyhow!("Failed to create clipboard context: {}", e))?;
        
        Ok(Self { ctx })
    }

    pub fn get_contents(&mut self) -> Result<String> {
        self.ctx
            .get_contents()
            .map_err(|e| anyhow::anyhow!("Failed to get clipboard contents: {}", e))
    }

    pub fn set_contents(&mut self, contents: &str) -> Result<()> {
        self.ctx
            .set_contents(contents.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to set clipboard contents: {}", e))
    }

    pub fn backup_and_set(&mut self, new_contents: &str) -> Result<String> {
        let backup = self.get_contents().unwrap_or_default();
        self.set_contents(new_contents)?;
        Ok(backup)
    }

    pub fn restore_contents(&mut self, contents: &str) -> Result<()> {
        self.set_contents(contents)
    }

    pub fn clear(&mut self) -> Result<()> {
        self.set_contents("")
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new().expect("Failed to create clipboard manager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_operations() -> Result<()> {
        let mut clipboard = ClipboardManager::new()?;
        
        // Test setting and getting contents
        let test_content = "Hello, Typely!";
        clipboard.set_contents(test_content)?;
        
        let retrieved = clipboard.get_contents()?;
        assert_eq!(retrieved, test_content);
        
        // Test backup and restore
        let original = "Original content";
        clipboard.set_contents(original)?;
        
        let backup = clipboard.backup_and_set("New content")?;
        assert_eq!(backup, original);
        
        let current = clipboard.get_contents()?;
        assert_eq!(current, "New content");
        
        clipboard.restore_contents(&backup)?;
        let restored = clipboard.get_contents()?;
        assert_eq!(restored, original);
        
        Ok(())
    }
}