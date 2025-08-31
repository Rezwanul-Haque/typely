use rdev::{Event, EventType, Key};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use anyhow::Result;

pub struct KeyboardMonitor {
    event_sender: Arc<Mutex<Option<Sender<KeyboardEvent>>>>,
    is_running: Arc<Mutex<bool>>,
}

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

impl KeyboardMonitor {
    pub fn new() -> Self {
        Self {
            event_sender: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start_monitoring(&self) -> Result<Receiver<KeyboardEvent>> {
        let (sender, receiver) = mpsc::channel();
        
        {
            let mut event_sender = self.event_sender.lock().unwrap();
            *event_sender = Some(sender);
        }
        
        {
            let mut is_running = self.is_running.lock().unwrap();
            *is_running = true;
        }

        let event_sender_clone = Arc::clone(&self.event_sender);
        let is_running_clone = Arc::clone(&self.is_running);

        thread::spawn(move || {
            let callback = move |event: Event| {
                let is_running = {
                    let is_running = is_running_clone.lock().unwrap();
                    *is_running
                };

                if !is_running {
                    return;
                }

                let keyboard_event = match Self::convert_event(event) {
                    Some(event) => event,
                    None => return,
                };

                let sender = {
                    let event_sender = event_sender_clone.lock().unwrap();
                    event_sender.clone()
                };

                if let Some(sender) = sender {
                    let _ = sender.send(keyboard_event);
                }
            };

            if let Err(error) = listen(callback) {
                eprintln!("Error listening to keyboard events: {:?}", error);
            }
        });

        Ok(receiver)
    }

    pub fn stop_monitoring(&self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        
        let mut event_sender = self.event_sender.lock().unwrap();
        *event_sender = None;
    }

    pub fn is_running(&self) -> bool {
        let is_running = self.is_running.lock().unwrap();
        *is_running
    }

    fn convert_event(event: Event) -> Option<KeyboardEvent> {
        let event_type = match event.event_type {
            EventType::KeyPress(_) => KeyboardEventType::KeyDown,
            EventType::KeyRelease(_) => KeyboardEventType::KeyUp,
            _ => return None,
        };

        let key_string = match event.event_type {
            EventType::KeyPress(key) | EventType::KeyRelease(key) => {
                Self::key_to_string(key)
            }
            _ => return None,
        };

        Some(KeyboardEvent {
            key: key_string,
            event_type,
        })
    }

    fn key_to_string(key: Key) -> String {
        match key {
            Key::Alt => "Alt".to_string(),
            Key::AltGr => "AltGr".to_string(),
            Key::Backspace => "Backspace".to_string(),
            Key::CapsLock => "CapsLock".to_string(),
            Key::ControlLeft => "ControlLeft".to_string(),
            Key::ControlRight => "ControlRight".to_string(),
            Key::Delete => "Delete".to_string(),
            Key::DownArrow => "DownArrow".to_string(),
            Key::End => "End".to_string(),
            Key::Escape => "Escape".to_string(),
            Key::F1 => "F1".to_string(),
            Key::F10 => "F10".to_string(),
            Key::F11 => "F11".to_string(),
            Key::F12 => "F12".to_string(),
            Key::F2 => "F2".to_string(),
            Key::F3 => "F3".to_string(),
            Key::F4 => "F4".to_string(),
            Key::F5 => "F5".to_string(),
            Key::F6 => "F6".to_string(),
            Key::F7 => "F7".to_string(),
            Key::F8 => "F8".to_string(),
            Key::F9 => "F9".to_string(),
            Key::Home => "Home".to_string(),
            Key::LeftArrow => "LeftArrow".to_string(),
            Key::MetaLeft => "MetaLeft".to_string(),
            Key::MetaRight => "MetaRight".to_string(),
            Key::PageDown => "PageDown".to_string(),
            Key::PageUp => "PageUp".to_string(),
            Key::Return => "Return".to_string(),
            Key::RightArrow => "RightArrow".to_string(),
            Key::ShiftLeft => "ShiftLeft".to_string(),
            Key::ShiftRight => "ShiftRight".to_string(),
            Key::Space => " ".to_string(),
            Key::Tab => "Tab".to_string(),
            Key::UpArrow => "UpArrow".to_string(),
            Key::PrintScreen => "PrintScreen".to_string(),
            Key::ScrollLock => "ScrollLock".to_string(),
            Key::Pause => "Pause".to_string(),
            Key::NumLock => "NumLock".to_string(),
            Key::BackQuote => "`".to_string(),
            Key::Num1 => "1".to_string(),
            Key::Num2 => "2".to_string(),
            Key::Num3 => "3".to_string(),
            Key::Num4 => "4".to_string(),
            Key::Num5 => "5".to_string(),
            Key::Num6 => "6".to_string(),
            Key::Num7 => "7".to_string(),
            Key::Num8 => "8".to_string(),
            Key::Num9 => "9".to_string(),
            Key::Num0 => "0".to_string(),
            Key::Minus => "-".to_string(),
            Key::Equal => "=".to_string(),
            Key::KeyQ => "q".to_string(),
            Key::KeyW => "w".to_string(),
            Key::KeyE => "e".to_string(),
            Key::KeyR => "r".to_string(),
            Key::KeyT => "t".to_string(),
            Key::KeyY => "y".to_string(),
            Key::KeyU => "u".to_string(),
            Key::KeyI => "i".to_string(),
            Key::KeyO => "o".to_string(),
            Key::KeyP => "p".to_string(),
            Key::LeftBracket => "[".to_string(),
            Key::RightBracket => "]".to_string(),
            Key::KeyA => "a".to_string(),
            Key::KeyS => "s".to_string(),
            Key::KeyD => "d".to_string(),
            Key::KeyF => "f".to_string(),
            Key::KeyG => "g".to_string(),
            Key::KeyH => "h".to_string(),
            Key::KeyJ => "j".to_string(),
            Key::KeyK => "k".to_string(),
            Key::KeyL => "l".to_string(),
            Key::SemiColon => ";".to_string(),
            Key::Quote => "'".to_string(),
            Key::BackSlash => "\\".to_string(),
            Key::IntlBackslash => "\\".to_string(),
            Key::KeyZ => "z".to_string(),
            Key::KeyX => "x".to_string(),
            Key::KeyC => "c".to_string(),
            Key::KeyV => "v".to_string(),
            Key::KeyB => "b".to_string(),
            Key::KeyN => "n".to_string(),
            Key::KeyM => "m".to_string(),
            Key::Comma => ",".to_string(),
            Key::Dot => ".".to_string(),
            Key::Slash => "/".to_string(),
            Key::Insert => "Insert".to_string(),
            Key::KpReturn => "KpReturn".to_string(),
            Key::KpMinus => "KpMinus".to_string(),
            Key::KpPlus => "KpPlus".to_string(),
            Key::KpMultiply => "KpMultiply".to_string(),
            Key::KpDivide => "KpDivide".to_string(),
            Key::Kp0 => "Kp0".to_string(),
            Key::Kp1 => "Kp1".to_string(),
            Key::Kp2 => "Kp2".to_string(),
            Key::Kp3 => "Kp3".to_string(),
            Key::Kp4 => "Kp4".to_string(),
            Key::Kp5 => "Kp5".to_string(),
            Key::Kp6 => "Kp6".to_string(),
            Key::Kp7 => "Kp7".to_string(),
            Key::Kp8 => "Kp8".to_string(),
            Key::Kp9 => "Kp9".to_string(),
            Key::KpDelete => "KpDelete".to_string(),
            Key::Function => "Function".to_string(),
            Key::Unknown(code) => format!("Unknown({})", code),
        }
    }
}

impl Default for KeyboardMonitor {
    fn default() -> Self {
        Self::new()
    }
}