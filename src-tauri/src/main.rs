// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{
    AppHandle, CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Window, WindowEvent,
};
use typely::application::services::TypelyService;
use typely::application::dto::*;
use typely::infrastructure::DatabaseConnection;
use typely::infrastructure::get_default_database_path;

// Application state
struct AppState {
    service: Arc<Mutex<TypelyService>>,
    window: Arc<Mutex<Option<tauri::Window>>>,
}

impl AppState {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize database connection
        let db_path = get_default_database_path()?;
        log::info!("Using database: {}", db_path.display());
        
        let db_connection = DatabaseConnection::new(&db_path).await?;
        let service = TypelyService::new(db_connection).await;

        Ok(Self {
            service: Arc::new(Mutex::new(service)),
            window: Arc::new(Mutex::new(None)),
        })
    }
}

// Tauri commands
#[tauri::command]
async fn get_snippets(state: State<'_, AppState>) -> Result<Vec<SnippetDto>, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let request = SnippetListRequest {
        search_term: None,
        tags: None,
        is_active: Some(true),
        limit: None,
        offset: None,
        sort_by: Some("updated".to_string()),
        sort_order: Some("desc".to_string()),
    };
    
    let response = service.list_snippets(request).await.map_err(|e| e.to_string())?;
    Ok(response.snippets)
}

#[tauri::command]
async fn create_snippet(
    trigger: String,
    replacement: String,
    tags: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<SnippetDto, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let request = CreateSnippetRequest {
        trigger,
        replacement,
        tags,
    };
    
    service.create_snippet(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_snippet(
    id: String,
    trigger: Option<String>,
    replacement: Option<String>,
    tags: Option<Vec<String>>,
    is_active: Option<bool>,
    state: State<'_, AppState>,
) -> Result<SnippetDto, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let snippet_id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let request = UpdateSnippetRequest {
        id: snippet_id,
        trigger,
        replacement,
        tags,
        is_active,
    };
    
    service.update_snippet(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_snippet(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let snippet_id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    service.delete_snippet(snippet_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn expand_snippet(trigger: String, state: State<'_, AppState>) -> Result<ExpansionResponse, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let request = ExpansionRequest {
        trigger,
        context: None,
    };
    
    service.expand_snippet(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_snippets(query: String, state: State<'_, AppState>) -> Result<Vec<SnippetDto>, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    service.search_snippets(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_statistics(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    
    // Get basic stats by calling different methods
    let all_snippets = service.get_all_active_snippets().await.map_err(|e| e.to_string())?;
    let total_snippets = all_snippets.len();
    let total_usage: u64 = all_snippets.iter().map(|s| s.usage_count).sum();
    
    // Get most used snippets
    let most_used = service.get_most_used_snippets(10).await.map_err(|e| e.to_string())?;
    
    // Create statistics object
    let stats = serde_json::json!({
        "total_snippets": total_snippets,
        "active_snippets": total_snippets,
        "inactive_snippets": 0,
        "total_usage": total_usage,
        "most_used": most_used,
    });
    
    Ok(stats)
}

#[tauri::command]
async fn export_snippets(state: State<'_, AppState>) -> Result<String, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let request = ExportSnippetsRequest {
        include_inactive: false,
        tags_filter: None,
    };
    
    service.export_to_json(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn import_snippets(json_data: String, overwrite: bool, state: State<'_, AppState>) -> Result<ImportResult, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    service.import_from_json(&json_data, overwrite).await.map_err(|e| e.to_string())
}

fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show Window");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Window");
    let new_snippet = CustomMenuItem::new("new_snippet".to_string(), "New Snippet");
    
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(new_snippet)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            // Show/hide window on left click
            let window = app.get_window("main").unwrap();
            if window.is_visible().unwrap() {
                let _ = window.hide();
            } else {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "show" => {
                let window = app.get_window("main").unwrap();
                let _ = window.show();
                let _ = window.set_focus();
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                let _ = window.hide();
            }
            "new_snippet" => {
                let window = app.get_window("main").unwrap();
                let _ = window.show();
                let _ = window.set_focus();
                // Emit event to frontend to show new snippet dialog
                let _ = window.emit("show-new-snippet", {});
            }
            _ => {}
        },
        _ => {}
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    
    // Initialize application state
    let app_state = AppState::new().await.expect("Failed to initialize application state");

    tauri::Builder::default()
        .manage(app_state)
        .system_tray(create_system_tray())
        .on_system_tray_event(handle_system_tray_event)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                // Hide window instead of closing when user clicks X
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_snippets,
            create_snippet,
            update_snippet,
            delete_snippet,
            expand_snippet,
            search_snippets,
            get_statistics,
            export_snippets,
            import_snippets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}