use anyhow::Result;
use std::sync::mpsc::Receiver;

// Stub implementations for when system integration is not available

#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key: String,
    pub event_type: KeyboardEventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardEventType {
    KeyDown,
    KeyUp,
}

pub struct KeyboardMonitor;

impl KeyboardMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn start_monitoring(&self) -> Result<Receiver<KeyboardEvent>> {
        Err(anyhow::anyhow!("System integration not available. Build with --features system-integration"))
    }

    pub fn stop_monitoring(&self) {
        // No-op
    }

    pub fn is_running(&self) -> bool {
        false
    }
}

pub struct ClipboardManager;

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        Err(anyhow::anyhow!("System integration not available. Build with --features system-integration"))
    }

    pub fn get_contents(&mut self) -> Result<String> {
        Err(anyhow::anyhow!("System integration not available"))
    }

    pub fn set_contents(&mut self, _contents: &str) -> Result<()> {
        Err(anyhow::anyhow!("System integration not available"))
    }

    pub fn backup_and_set(&mut self, _new_contents: &str) -> Result<String> {
        Err(anyhow::anyhow!("System integration not available"))
    }

    pub fn restore_contents(&mut self, _contents: &str) -> Result<()> {
        Err(anyhow::anyhow!("System integration not available"))
    }

    pub fn clear(&mut self) -> Result<()> {
        Err(anyhow::anyhow!("System integration not available"))
    }
}

pub struct InputSimulator;

impl InputSimulator {
    pub fn new() -> Result<Self> {
        Err(anyhow::anyhow!("System integration not available. Build with --features system-integration"))
    }

    pub fn type_text(&mut self, _text: &str) -> Result<()> {
        Err(anyhow::anyhow!("System integration not available"))
    }

    pub fn replace_trigger_with_expansion(&mut self, _trigger_length: usize, _expansion: &str) -> Result<()> {
        Err(anyhow::anyhow!("System integration not available"))
    }
}