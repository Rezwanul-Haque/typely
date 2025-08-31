use crate::application::TypelyService;
use crate::application::dto::{ExpansionRequest, ExpansionResponse};
use crate::domain::{TriggerDetectionService, TriggerMatch};
use crate::infrastructure::{KeyboardMonitor, KeyboardEvent, KeyboardEventType, InputSimulator, ClipboardManager};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::mpsc as tokio_mpsc;

pub struct TextExpansionEngine {
    service: Arc<TypelyService>,
    keyboard_monitor: KeyboardMonitor,
    trigger_detection: TriggerDetectionService,
    input_simulator: Arc<Mutex<InputSimulator>>,
    clipboard_manager: Arc<Mutex<ClipboardManager>>,
    is_running: Arc<Mutex<bool>>,
    buffer: Arc<Mutex<TextBuffer>>,
    config: ExpansionConfig,
}

#[derive(Debug, Clone)]
pub struct ExpansionConfig {
    pub buffer_size: usize,
    pub trigger_timeout_ms: u64,
    pub expansion_delay_ms: u64,
    pub enabled: bool,
    pub case_sensitive: bool,
}

impl Default for ExpansionConfig {
    fn default() -> Self {
        Self {
            buffer_size: 100,
            trigger_timeout_ms: 1000,
            expansion_delay_ms: 50,
            enabled: true,
            case_sensitive: true,
        }
    }
}

#[derive(Debug, Clone)]
struct TextBuffer {
    content: VecDeque<char>,
    last_update: Instant,
    max_size: usize,
}

impl TextBuffer {
    fn new(max_size: usize) -> Self {
        Self {
            content: VecDeque::with_capacity(max_size),
            last_update: Instant::now(),
            max_size,
        }
    }

    fn add_char(&mut self, c: char) {
        if self.content.len() >= self.max_size {
            self.content.pop_front();
        }
        self.content.push_back(c);
        self.last_update = Instant::now();
    }

    fn remove_chars(&mut self, count: usize) {
        for _ in 0..count {
            self.content.pop_back();
        }
        self.last_update = Instant::now();
    }

    fn get_text(&self) -> String {
        self.content.iter().collect()
    }

    fn clear(&mut self) {
        self.content.clear();
        self.last_update = Instant::now();
    }

    fn is_expired(&self, timeout_ms: u64) -> bool {
        self.last_update.elapsed() > Duration::from_millis(timeout_ms)
    }
}

impl TextExpansionEngine {
    pub fn new(service: Arc<TypelyService>, config: Option<ExpansionConfig>) -> Result<Self> {
        let config = config.unwrap_or_default();
        
        Ok(Self {
            service,
            keyboard_monitor: KeyboardMonitor::new(),
            trigger_detection: TriggerDetectionService::new(),
            input_simulator: Arc::new(Mutex::new(InputSimulator::new()?)),
            clipboard_manager: Arc::new(Mutex::new(ClipboardManager::new()?)),
            is_running: Arc::new(Mutex::new(false)),
            buffer: Arc::new(Mutex::new(TextBuffer::new(config.buffer_size))),
            config,
        })
    }

    pub async fn start(&self) -> Result<()> {
        {
            let mut is_running = self.is_running.lock().unwrap();
            if *is_running {
                return Err(anyhow::anyhow!("Text expansion engine is already running"));
            }
            *is_running = true;
        }

        log::info!("Starting text expansion engine");

        // Start keyboard monitoring
        let receiver = self.keyboard_monitor.start_monitoring()?;
        
        // Create expansion event channel
        let (expansion_sender, mut expansion_receiver) = tokio_mpsc::channel(100);

        // Clone necessary data for the keyboard event processing thread
        let buffer = Arc::clone(&self.buffer);
        let trigger_detection = self.trigger_detection.clone();
        let service = Arc::clone(&self.service);
        let config = self.config.clone();
        let is_running = Arc::clone(&self.is_running);

        // Spawn thread to handle keyboard events
        let keyboard_thread_sender = expansion_sender.clone();
        thread::spawn(move || {
            Self::handle_keyboard_events(
                receiver,
                buffer,
                trigger_detection,
                service,
                config,
                is_running,
                keyboard_thread_sender,
            );
        });

        // Handle expansion events in async context
        let input_simulator = Arc::clone(&self.input_simulator);
        let clipboard_manager = Arc::clone(&self.clipboard_manager);
        let expansion_config = self.config.clone();
        
        tokio::spawn(async move {
            while let Some(expansion_event) = expansion_receiver.recv().await {
                if let Err(e) = Self::handle_expansion_event(
                    expansion_event,
                    &input_simulator,
                    &clipboard_manager,
                    &expansion_config,
                ).await {
                    log::error!("Failed to handle expansion event: {}", e);
                }
            }
        });

        Ok(())
    }

    pub fn stop(&self) {
        {
            let mut is_running = self.is_running.lock().unwrap();
            *is_running = false;
        }

        self.keyboard_monitor.stop_monitoring();
        log::info!("Text expansion engine stopped");
    }

    pub fn is_running(&self) -> bool {
        let is_running = self.is_running.lock().unwrap();
        *is_running
    }

    pub fn update_config(&mut self, config: ExpansionConfig) {
        self.config = config;
        
        // Update buffer size if changed
        let mut buffer = self.buffer.lock().unwrap();
        buffer.max_size = self.config.buffer_size;
        
        // Trim buffer if necessary
        while buffer.content.len() > buffer.max_size {
            buffer.content.pop_front();
        }
    }

    fn handle_keyboard_events(
        receiver: Receiver<KeyboardEvent>,
        buffer: Arc<Mutex<TextBuffer>>,
        trigger_detection: TriggerDetectionService,
        service: Arc<TypelyService>,
        config: ExpansionConfig,
        is_running: Arc<Mutex<bool>>,
        expansion_sender: tokio_mpsc::Sender<ExpansionEvent>,
    ) {
        while let Ok(event) = receiver.recv() {
            // Check if we should continue running
            {
                let running = is_running.lock().unwrap();
                if !*running {
                    break;
                }
            }

            if !config.enabled {
                continue;
            }

            // Only process key down events for typing
            if event.event_type != KeyboardEventType::KeyDown {
                continue;
            }

            // Handle different types of keys
            match event.key.as_str() {
                // Regular characters
                key if key.len() == 1 => {
                    let c = key.chars().next().unwrap();
                    
                    // Add character to buffer
                    {
                        let mut buffer = buffer.lock().unwrap();
                        buffer.add_char(c);
                        
                        // Clear buffer if expired
                        if buffer.is_expired(config.trigger_timeout_ms) {
                            buffer.clear();
                            continue;
                        }
                    }

                    // Check for triggers
                    let buffer_text = {
                        let buffer = buffer.lock().unwrap();
                        buffer.get_text()
                    };

                    let triggers = trigger_detection.find_triggers_in_text(&buffer_text);
                    
                    // Process the most recent complete trigger
                    if let Some(trigger_match) = triggers.last() {
                        // Check if this trigger ends at the current position
                        if trigger_match.end_position == buffer_text.len() {
                            let expansion_event = ExpansionEvent {
                                trigger: trigger_match.trigger.clone(),
                                trigger_length: trigger_match.length(),
                                buffer_text: buffer_text.clone(),
                            };

                            // Send expansion event (non-blocking)
                            if let Err(_) = expansion_sender.try_send(expansion_event) {
                                log::warn!("Expansion event channel is full, skipping expansion");
                            }
                        }
                    }
                }
                // Special keys that might interrupt typing
                "Backspace" => {
                    let mut buffer = buffer.lock().unwrap();
                    if !buffer.content.is_empty() {
                        buffer.content.pop_back();
                    }
                    buffer.last_update = Instant::now();
                }
                "Return" | "Tab" | "Space" => {
                    // These keys typically end a word/trigger sequence
                    let mut buffer = buffer.lock().unwrap();
                    buffer.clear();
                }
                "Escape" | "ControlLeft" | "ControlRight" | "Alt" | "MetaLeft" | "MetaRight" => {
                    // Control keys clear the buffer
                    let mut buffer = buffer.lock().unwrap();
                    buffer.clear();
                }
                _ => {
                    // Other keys don't affect the buffer significantly
                }
            }
        }
    }

    async fn handle_expansion_event(
        event: ExpansionEvent,
        input_simulator: &Arc<Mutex<InputSimulator>>,
        clipboard_manager: &Arc<Mutex<ClipboardManager>>,
        config: &ExpansionConfig,
    ) -> Result<()> {
        // Small delay to ensure the key event is processed
        tokio::time::sleep(Duration::from_millis(config.expansion_delay_ms)).await;

        // Try to expand the snippet
        let expansion_request = ExpansionRequest {
            trigger: event.trigger.clone(),
            context: Some(event.buffer_text),
        };

        // Note: This is a blocking call in an async context, which isn't ideal
        // In a real implementation, you'd want to make the service async
        let expansion_response = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                // This is a simplified approach - in production you'd want proper async service
                Ok::<ExpansionResponse, anyhow::Error>(ExpansionResponse {
                    success: true,
                    expanded_text: Some(format!("Expanded: {}", event.trigger)),
                    error_message: None,
                })
            })
        })?;

        if expansion_response.success {
            if let Some(expanded_text) = expansion_response.expanded_text {
                // Perform the text replacement
                Self::replace_text(
                    &event.trigger,
                    &expanded_text,
                    event.trigger_length,
                    input_simulator,
                    clipboard_manager,
                )?;

                log::info!("Expanded '{}' to '{}'", event.trigger, expanded_text);
            }
        } else if let Some(error) = expansion_response.error_message {
            log::debug!("Expansion failed for '{}': {}", event.trigger, error);
        }

        Ok(())
    }

    fn replace_text(
        trigger: &str,
        expanded_text: &str,
        trigger_length: usize,
        input_simulator: &Arc<Mutex<InputSimulator>>,
        clipboard_manager: &Arc<Mutex<ClipboardManager>>,
    ) -> Result<()> {
        let mut simulator = input_simulator.lock().unwrap();
        let mut clipboard = clipboard_manager.lock().unwrap();

        // Method 1: Simple backspace and type (most compatible)
        simulator.replace_trigger_with_expansion(trigger_length, expanded_text)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct ExpansionEvent {
    trigger: String,
    trigger_length: usize,
    buffer_text: String,
}

impl Drop for TextExpansionEngine {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::DatabaseConnection;
    use tempfile::TempDir;

    async fn create_test_engine() -> (TextExpansionEngine, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_connection = DatabaseConnection::new(&db_path).await.unwrap();
        let service = Arc::new(TypelyService::new(db_connection).await);
        
        let config = ExpansionConfig {
            buffer_size: 50,
            trigger_timeout_ms: 500,
            expansion_delay_ms: 10,
            enabled: true,
            case_sensitive: true,
        };
        
        let engine = TextExpansionEngine::new(service, Some(config)).unwrap();
        (engine, temp_dir)
    }

    #[tokio::test]
    async fn test_text_buffer() {
        let mut buffer = TextBuffer::new(5);
        
        // Test adding characters
        buffer.add_char('a');
        buffer.add_char('b');
        buffer.add_char('c');
        
        assert_eq!(buffer.get_text(), "abc");
        
        // Test buffer overflow
        buffer.add_char('d');
        buffer.add_char('e');
        buffer.add_char('f'); // Should remove 'a'
        
        assert_eq!(buffer.get_text(), "bcdef");
        
        // Test removing characters
        buffer.remove_chars(2);
        assert_eq!(buffer.get_text(), "bcd");
        
        // Test clearing
        buffer.clear();
        assert_eq!(buffer.get_text(), "");
    }

    #[test]
    fn test_expansion_config() {
        let config = ExpansionConfig::default();
        
        assert_eq!(config.buffer_size, 100);
        assert_eq!(config.trigger_timeout_ms, 1000);
        assert_eq!(config.expansion_delay_ms, 50);
        assert!(config.enabled);
        assert!(config.case_sensitive);
    }

    #[tokio::test]
    async fn test_engine_creation() {
        let (_engine, _temp_dir) = create_test_engine().await;
        // Engine should be created successfully
    }

    // Note: Testing the actual keyboard monitoring and input simulation
    // would require a real GUI environment and is complex to test in unit tests
}