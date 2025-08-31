use enigo::{Enigo, Key, Keyboard, Settings};
use anyhow::Result;
use std::thread;
use std::time::Duration;

pub struct InputSimulator {
    enigo: Enigo,
}

impl InputSimulator {
    pub fn new() -> Result<Self> {
        let enigo = Enigo::new(&Settings::default())
            .map_err(|e| anyhow::anyhow!("Failed to create input simulator: {}", e))?;
        
        Ok(Self { enigo })
    }

    pub fn type_text(&mut self, text: &str) -> Result<()> {
        // Small delay to ensure the system is ready
        thread::sleep(Duration::from_millis(10));
        
        self.enigo
            .text(text)
            .map_err(|e| anyhow::anyhow!("Failed to type text: {}", e))?;
        
        Ok(())
    }

    pub fn type_text_with_delay(&mut self, text: &str, delay_ms: u64) -> Result<()> {
        thread::sleep(Duration::from_millis(delay_ms));
        self.type_text(text)
    }

    pub fn press_key(&mut self, key: Key) -> Result<()> {
        self.enigo
            .key(key, enigo::Direction::Press)
            .map_err(|e| anyhow::anyhow!("Failed to press key: {}", e))?;
        
        Ok(())
    }

    pub fn release_key(&mut self, key: Key) -> Result<()> {
        self.enigo
            .key(key, enigo::Direction::Release)
            .map_err(|e| anyhow::anyhow!("Failed to release key: {}", e))?;
        
        Ok(())
    }

    pub fn key_click(&mut self, key: Key) -> Result<()> {
        self.enigo
            .key(key, enigo::Direction::Click)
            .map_err(|e| anyhow::anyhow!("Failed to click key: {}", e))?;
        
        Ok(())
    }

    pub fn send_backspaces(&mut self, count: usize) -> Result<()> {
        for _ in 0..count {
            self.key_click(Key::Backspace)?;
            // Small delay between backspaces
            thread::sleep(Duration::from_millis(1));
        }
        Ok(())
    }

    pub fn select_all(&mut self) -> Result<()> {
        self.press_key(Key::Control)?;
        self.key_click(Key::Unicode('a'))?;
        self.release_key(Key::Control)?;
        Ok(())
    }

    pub fn copy(&mut self) -> Result<()> {
        self.press_key(Key::Control)?;
        self.key_click(Key::Unicode('c'))?;
        self.release_key(Key::Control)?;
        Ok(())
    }

    pub fn paste(&mut self) -> Result<()> {
        self.press_key(Key::Control)?;
        self.key_click(Key::Unicode('v'))?;
        self.release_key(Key::Control)?;
        Ok(())
    }

    pub fn cut(&mut self) -> Result<()> {
        self.press_key(Key::Control)?;
        self.key_click(Key::Unicode('x'))?;
        self.release_key(Key::Control)?;
        Ok(())
    }

    pub fn undo(&mut self) -> Result<()> {
        self.press_key(Key::Control)?;
        self.key_click(Key::Unicode('z'))?;
        self.release_key(Key::Control)?;
        Ok(())
    }

    pub fn replace_trigger_with_expansion(&mut self, trigger_length: usize, expansion: &str) -> Result<()> {
        // Delete the trigger by sending backspaces
        self.send_backspaces(trigger_length)?;
        
        // Type the expansion
        self.type_text(expansion)?;
        
        Ok(())
    }

    pub fn replace_selection_with_text(&mut self, text: &str) -> Result<()> {
        // Simply type the text - it will replace selected text
        self.type_text(text)?;
        Ok(())
    }
}

impl Default for InputSimulator {
    fn default() -> Self {
        Self::new().expect("Failed to create input simulator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would require actual GUI interaction and are difficult to test
    // in a headless environment. They are here as examples of how to use the API.
    
    #[test]
    fn test_input_simulator_creation() {
        let result = InputSimulator::new();
        // This might fail in headless environments
        if result.is_ok() {
            println!("InputSimulator created successfully");
        } else {
            println!("InputSimulator creation failed (expected in headless environment)");
        }
    }
}