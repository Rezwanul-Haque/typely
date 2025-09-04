use crate::infra::{KeyboardEvent, KeyboardEventType};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct KeyboardEventHandler {
    key_states: HashMap<String, KeyState>,
    modifiers: ModifierState,
    last_activity: Instant,
}

#[derive(Debug, Clone)]
struct KeyState {
    is_pressed: bool,
    press_time: Instant,
    release_time: Option<Instant>,
}

#[derive(Debug, Clone, Default)]
pub struct ModifierState {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Debug, Clone)]
pub struct ProcessedKeyEvent {
    pub key: String,
    pub event_type: KeyboardEventType,
    pub modifiers: ModifierState,
    pub is_modifier: bool,
    pub is_printable: bool,
    pub should_buffer: bool,
}

impl KeyboardEventHandler {
    pub fn new() -> Self {
        Self {
            key_states: HashMap::new(),
            modifiers: ModifierState::default(),
            last_activity: Instant::now(),
        }
    }

    pub fn process_event(&mut self, event: KeyboardEvent) -> Option<ProcessedKeyEvent> {
        self.last_activity = Instant::now();

        // Update key state
        self.update_key_state(&event);

        // Update modifier state
        self.update_modifier_state(&event);

        // Create processed event
        let processed = ProcessedKeyEvent {
            key: event.key.clone(),
            event_type: event.event_type.clone(),
            modifiers: self.modifiers.clone(),
            is_modifier: self.is_modifier_key(&event.key),
            is_printable: self.is_printable_key(&event.key),
            should_buffer: self.should_buffer_key(&event),
        };

        Some(processed)
    }

    pub fn is_idle(&self, timeout: Duration) -> bool {
        self.last_activity.elapsed() > timeout
    }

    pub fn reset(&mut self) {
        self.key_states.clear();
        self.modifiers = ModifierState::default();
        self.last_activity = Instant::now();
    }

    pub fn get_modifier_state(&self) -> &ModifierState {
        &self.modifiers
    }

    pub fn is_key_pressed(&self, key: &str) -> bool {
        self.key_states
            .get(key)
            .map(|state| state.is_pressed)
            .unwrap_or(false)
    }

    fn update_key_state(&mut self, event: &KeyboardEvent) {
        let key_state = self
            .key_states
            .entry(event.key.clone())
            .or_insert(KeyState {
                is_pressed: false,
                press_time: Instant::now(),
                release_time: None,
            });

        match event.event_type {
            KeyboardEventType::KeyDown => {
                if !key_state.is_pressed {
                    key_state.is_pressed = true;
                    key_state.press_time = Instant::now();
                    key_state.release_time = None;
                }
            }
            KeyboardEventType::KeyUp => {
                key_state.is_pressed = false;
                key_state.release_time = Some(Instant::now());
            }
            KeyboardEventType::KeyPress => {
                // Handle key press event (combination of down and up)
                key_state.is_pressed = false;
                key_state.press_time = Instant::now();
                key_state.release_time = Some(Instant::now());
            }
        }
    }

    fn update_modifier_state(&mut self, event: &KeyboardEvent) {
        match event.key.as_str() {
            "ControlLeft" | "ControlRight" => {
                self.modifiers.ctrl = event.event_type == KeyboardEventType::KeyDown;
            }
            "ShiftLeft" | "ShiftRight" => {
                self.modifiers.shift = event.event_type == KeyboardEventType::KeyDown;
            }
            "Alt" | "AltGr" => {
                self.modifiers.alt = event.event_type == KeyboardEventType::KeyDown;
            }
            "MetaLeft" | "MetaRight" => {
                self.modifiers.meta = event.event_type == KeyboardEventType::KeyDown;
            }
            _ => {}
        }
    }

    fn is_modifier_key(&self, key: &str) -> bool {
        matches!(
            key,
            "ControlLeft"
                | "ControlRight"
                | "ShiftLeft"
                | "ShiftRight"
                | "Alt"
                | "AltGr"
                | "MetaLeft"
                | "MetaRight"
        )
    }

    fn is_printable_key(&self, key: &str) -> bool {
        // Check if the key produces a printable character
        if key.len() == 1 {
            let c = key.chars().next().unwrap();
            return c.is_ascii_graphic() || c == ' ';
        }

        // Special printable keys
        matches!(key, "Space" | "Tab" | "Return")
    }

    fn should_buffer_key(&self, event: &KeyboardEvent) -> bool {
        // Only buffer key down events for printable characters
        if event.event_type != KeyboardEventType::KeyDown {
            return false;
        }

        // Don't buffer if modifiers are pressed (except Shift for capitalization)
        if self.modifiers.ctrl || self.modifiers.alt || self.modifiers.meta {
            return false;
        }

        self.is_printable_key(&event.key)
    }

    pub fn get_pressed_keys(&self) -> Vec<String> {
        self.key_states
            .iter()
            .filter(|(_, state)| state.is_pressed)
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn has_modifiers(&self) -> bool {
        self.modifiers.ctrl || self.modifiers.shift || self.modifiers.alt || self.modifiers.meta
    }
}

impl Default for KeyboardEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ModifierState {
    pub fn has_any(&self) -> bool {
        self.ctrl || self.shift || self.alt || self.meta
    }

    pub fn is_ctrl_only(&self) -> bool {
        self.ctrl && !self.shift && !self.alt && !self.meta
    }

    pub fn is_shift_only(&self) -> bool {
        !self.ctrl && self.shift && !self.alt && !self.meta
    }

    pub fn is_alt_only(&self) -> bool {
        !self.ctrl && !self.shift && self.alt && !self.meta
    }

    pub fn matches(&self, other: &ModifierState) -> bool {
        self.ctrl == other.ctrl
            && self.shift == other.shift
            && self.alt == other.alt
            && self.meta == other.meta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_state() {
        let mut handler = KeyboardEventHandler::new();

        // Test Ctrl key
        let ctrl_down = KeyboardEvent {
            key: "ControlLeft".to_string(),
            event_type: KeyboardEventType::KeyDown,
            key_code: 0,
            text: None,
            timestamp: chrono::Utc::now(),
        };

        handler.process_event(ctrl_down);
        assert!(handler.get_modifier_state().ctrl);
        assert!(!handler.get_modifier_state().shift);

        let ctrl_up = KeyboardEvent {
            key: "ControlLeft".to_string(),
            event_type: KeyboardEventType::KeyUp,
            key_code: 0,
            text: None,
            timestamp: chrono::Utc::now(),
        };

        handler.process_event(ctrl_up);
        assert!(!handler.get_modifier_state().ctrl);
    }

    #[test]
    fn test_printable_key_detection() {
        let handler = KeyboardEventHandler::new();

        assert!(handler.is_printable_key("a"));
        assert!(handler.is_printable_key("A"));
        assert!(handler.is_printable_key("1"));
        assert!(handler.is_printable_key(" "));
        assert!(handler.is_printable_key("Space"));

        assert!(!handler.is_printable_key("ControlLeft"));
        assert!(!handler.is_printable_key("Escape"));
        assert!(!handler.is_printable_key("F1"));
    }

    #[test]
    fn test_should_buffer_key() {
        let mut handler = KeyboardEventHandler::new();

        // Regular character should be buffered
        let char_event = KeyboardEvent {
            key: "a".to_string(),
            event_type: KeyboardEventType::KeyDown,
            key_code: 0,
            text: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(handler.should_buffer_key(&char_event));

        // Key up events should not be buffered
        let char_up = KeyboardEvent {
            key: "a".to_string(),
            event_type: KeyboardEventType::KeyUp,
            key_code: 0,
            text: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(!handler.should_buffer_key(&char_up));

        // Set Ctrl modifier
        let ctrl_down = KeyboardEvent {
            key: "ControlLeft".to_string(),
            event_type: KeyboardEventType::KeyDown,
            key_code: 0,
            text: None,
            timestamp: chrono::Utc::now(),
        };
        handler.process_event(ctrl_down);

        // Character with Ctrl should not be buffered
        assert!(!handler.should_buffer_key(&char_event));
    }

    #[test]
    fn test_key_state_tracking() {
        let mut handler = KeyboardEventHandler::new();

        let key_down = KeyboardEvent {
            key: "a".to_string(),
            event_type: KeyboardEventType::KeyDown,
            key_code: 0,
            text: None,
            timestamp: chrono::Utc::now(),
        };

        handler.process_event(key_down);
        assert!(handler.is_key_pressed("a"));

        let key_up = KeyboardEvent {
            key: "a".to_string(),
            event_type: KeyboardEventType::KeyUp,
            key_code: 0,
            text: None,
            timestamp: chrono::Utc::now(),
        };

        handler.process_event(key_up);
        assert!(!handler.is_key_pressed("a"));
    }

    #[test]
    fn test_reset() {
        let mut handler = KeyboardEventHandler::new();

        // Press some keys
        let events = vec![
            KeyboardEvent {
                key: "ControlLeft".to_string(),
                event_type: KeyboardEventType::KeyDown,
                key_code: 0,
                text: None,
                timestamp: chrono::Utc::now(),
            },
            KeyboardEvent {
                key: "a".to_string(),
                event_type: KeyboardEventType::KeyDown,
                key_code: 0,
                text: None,
                timestamp: chrono::Utc::now(),
            },
        ];

        for event in events {
            handler.process_event(event);
        }

        assert!(handler.get_modifier_state().ctrl);
        assert!(handler.is_key_pressed("a"));

        // Reset should clear everything
        handler.reset();

        assert!(!handler.get_modifier_state().ctrl);
        assert!(!handler.is_key_pressed("a"));
    }
}
