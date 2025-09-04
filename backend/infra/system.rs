/// Stub implementations for system integration (CLI-only builds)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyboardEventType {
    KeyDown,
    KeyUp,
    KeyPress,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardEvent {
    pub event_type: KeyboardEventType,
    pub key_code: u32,
    pub key: String, // Add key field for compatibility
    pub text: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl KeyboardEvent {
    pub fn new(event_type: KeyboardEventType, key_code: u32) -> Self {
        Self {
            event_type,
            key_code,
            key: format!("{}", key_code), // Default key representation
            text: None,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Stub keyboard monitor (no-op for CLI builds)
pub struct KeyboardMonitor;

impl Default for KeyboardMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn start(&self) -> anyhow::Result<()> {
        // No-op for CLI builds
        Ok(())
    }

    pub fn stop(&self) -> anyhow::Result<()> {
        // No-op for CLI builds
        Ok(())
    }

    pub fn start_monitoring(&self) -> anyhow::Result<std::sync::mpsc::Receiver<KeyboardEvent>> {
        // Return empty channel for CLI builds
        let (_sender, receiver) = std::sync::mpsc::channel();
        Ok(receiver)
    }

    pub fn stop_monitoring(&self) {
        // No-op for CLI builds
    }
}

/// Stub input simulator (no-op for CLI builds)
pub struct InputSimulator;

impl InputSimulator {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub fn type_text(&self, _text: &str) -> anyhow::Result<()> {
        // No-op for CLI builds
        Ok(())
    }

    pub fn simulate_backspace(&self, _count: usize) -> anyhow::Result<()> {
        // No-op for CLI builds
        Ok(())
    }

    pub fn replace_trigger_with_expansion(
        &self,
        _trigger_length: usize,
        _expanded_text: &str,
    ) -> anyhow::Result<()> {
        // No-op for CLI builds
        Ok(())
    }
}

/// Stub clipboard manager (no-op for CLI builds)
pub struct ClipboardManager;

impl ClipboardManager {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub fn get_text(&self) -> anyhow::Result<String> {
        // Return empty string for CLI builds
        Ok(String::new())
    }

    pub fn set_text(&self, _text: &str) -> anyhow::Result<()> {
        // No-op for CLI builds
        Ok(())
    }
}
