// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use tauri::{
    AppHandle, CustomMenuItem, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};
use typely::app::services::TypelyService;
use typely::app::dto::*;
use typely::infra::DatabaseConnection;
use typely::infra::get_default_database_path;

#[derive(serde::Serialize)]
struct CliStatus {
    installed: bool,
    version: Option<String>,
    path: Option<String>,
}

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
    let request = SnippetListRequest {
        search_term: None,
        tags: None,
        is_active: Some(true),
        limit: None,
        offset: None,
        sort_by: Some("updated".to_string()),
        sort_order: Some("desc".to_string()),
    };
    
    let service = state.service.lock().map_err(|e| e.to_string())?;
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
    let request = CreateSnippetRequest {
        trigger,
        replacement,
        tags,
    };
    
    let service = state.service.lock().map_err(|e| e.to_string())?;
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
    let snippet_id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let request = UpdateSnippetRequest {
        id: snippet_id,
        trigger,
        replacement,
        tags,
        is_active,
    };
    
    let service = state.service.lock().map_err(|e| e.to_string())?;
    service.update_snippet(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_snippet(id: String, state: State<'_, AppState>) -> Result<bool, String> {
    let snippet_id = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    let service = state.service.lock().map_err(|e| e.to_string())?;
    service.delete_snippet(snippet_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn expand_snippet(trigger: String, state: State<'_, AppState>) -> Result<ExpansionResponse, String> {
    let request = ExpansionRequest {
        trigger,
        context: None,
    };
    
    let service = state.service.lock().map_err(|e| e.to_string())?;
    service.expand_snippet(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_snippets(query: String, state: State<'_, AppState>) -> Result<Vec<SnippetDto>, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    service.search_snippets(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_statistics(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let all_snippets = {
        let service = state.service.lock().map_err(|e| e.to_string())?;
        service.get_all_active_snippets().await.map_err(|e| e.to_string())?
    };
    
    let total_snippets = all_snippets.len();
    let total_usage: u64 = all_snippets.iter().map(|s| s.usage_count).sum();
    
    // Get most used snippets
    let most_used = {
        let service = state.service.lock().map_err(|e| e.to_string())?;
        service.get_most_used_snippets(10).await.map_err(|e| e.to_string())?
    };
    
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
    let request = ExportSnippetsRequest {
        include_inactive: false,
        tags_filter: None,
    };
    
    let service = state.service.lock().map_err(|e| e.to_string())?;
    service.export_to_json(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn import_snippets(json_data: String, overwrite: bool, state: State<'_, AppState>) -> Result<ImportResult, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    service.import_from_json(&json_data, overwrite).await.map_err(|e| e.to_string())
}

// CLI installation commands
#[tauri::command]
async fn check_cli_status() -> Result<CliStatus, String> {
    // Check if CLI is installed and accessible
    match which::which("typely-cli") {
        Ok(path) => {
            // Try to get version
            let version = Command::new("typely-cli")
                .arg("--version")
                .output()
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string()
                })
                .ok();
            
            Ok(CliStatus {
                installed: true,
                version,
                path: Some(path.to_string_lossy().to_string()),
            })
        }
        Err(_) => Ok(CliStatus {
            installed: false,
            version: None,
            path: None,
        }),
    }
}

#[tauri::command]
async fn install_cli() -> Result<String, String> {
    let cli_binary_name = if cfg!(windows) {
        "typely-cli.exe"
    } else {
        "typely-cli"
    };

    // Get the CLI binary path - it should be in the same directory as the GUI binary
    let current_exe = std::env::current_exe().map_err(|e| format!("Failed to get current executable path: {}", e))?;
    let current_dir = current_exe.parent().ok_or("Failed to get parent directory")?;
    let cli_source = current_dir.join(cli_binary_name);

    if !cli_source.exists() {
        return Err(format!("CLI binary not found at: {}", cli_source.display()));
    }

    // Determine installation target based on platform
    let install_path = get_cli_install_path()?;
    
    // Create target directory if it doesn't exist
    if let Some(parent) = install_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
    }

    // Copy CLI binary to install location
    fs::copy(&cli_source, &install_path).map_err(|e| {
        format!("Failed to copy CLI binary from {} to {}: {}", 
                cli_source.display(), install_path.display(), e)
    })?;

    // Make executable on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&install_path)
            .map_err(|e| format!("Failed to get permissions: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&install_path, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
    }

    // Update PATH if necessary
    update_user_path(&install_path)?;

    Ok(format!("CLI installed successfully to: {}", install_path.display()))
}

fn get_cli_install_path() -> Result<PathBuf, String> {
    let cli_binary_name = if cfg!(windows) {
        "typely-cli.exe"
    } else {
        "typely-cli"
    };

    if cfg!(windows) {
        // Windows: Install to Program Files
        let program_files = std::env::var("PROGRAMFILES")
            .unwrap_or_else(|_| "C:\\Program Files".to_string());
        let typely_dir = Path::new(&program_files).join("Typely");
        Ok(typely_dir.join(cli_binary_name))
    } else {
        // Unix-like: Try system-wide first, fallback to user directory
        let system_path = Path::new("/usr/local/bin").join(cli_binary_name);
        
        // Check if we have write permission to /usr/local/bin
        if Path::new("/usr/local/bin").exists() && 
           fs::metadata("/usr/local/bin").map(|m| !m.permissions().readonly()).unwrap_or(false) {
            Ok(system_path)
        } else {
            // Fallback to user's local bin
            let home = std::env::var("HOME").map_err(|_| "HOME environment variable not set")?;
            let local_bin = Path::new(&home).join(".local/bin");
            Ok(local_bin.join(cli_binary_name))
        }
    }
}

fn update_user_path(install_path: &Path) -> Result<(), String> {
    let install_dir = install_path.parent()
        .ok_or("Failed to get parent directory")?
        .to_string_lossy();

    #[cfg(windows)]
    {
        // On Windows, update user PATH via registry (requires restart or re-login)
        // For now, just inform the user
        log::info!("CLI installed to: {}. You may need to restart your terminal or add this directory to your PATH.", install_dir);
    }

    #[cfg(unix)]
    {
        // On Unix, try to update shell profile files
        let home = std::env::var("HOME").map_err(|_| "HOME environment variable not set")?;
        let shell_profiles = vec![
            Path::new(&home).join(".bashrc"),
            Path::new(&home).join(".zshrc"),
            Path::new(&home).join(".profile"),
        ];

        let path_line = format!("export PATH=\"$PATH:{}\"", install_dir);
        
        for profile in shell_profiles {
            if profile.exists() {
                // Check if PATH is already updated
                if let Ok(content) = fs::read_to_string(&profile) {
                    if !content.contains(&install_dir) {
                        // Append to profile file
                        fs::write(&profile, format!("{}\n# Added by Typely\n{}\n", content, path_line))
                            .map_err(|e| format!("Failed to update {}: {}", profile.display(), e))?;
                        log::info!("Updated PATH in: {}", profile.display());
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
async fn uninstall_cli() -> Result<String, String> {
    // Find CLI installation
    match which::which("typely-cli") {
        Ok(cli_path) => {
            // Remove the binary
            fs::remove_file(&cli_path)
                .map_err(|e| format!("Failed to remove CLI binary: {}", e))?;
            
            Ok(format!("CLI uninstalled from: {}", cli_path.display()))
        }
        Err(_) => Err("CLI is not installed or not in PATH".to_string()),
    }
}

#[tauri::command]
async fn open_terminal_with_cli() -> Result<String, String> {
    let cli_status = check_cli_status().await?;
    if !cli_status.installed {
        return Err("CLI is not installed. Please install it first.".to_string());
    }

    // Open terminal with CLI ready
    #[cfg(windows)]
    {
        Command::new("cmd")
            .args(&["/C", "start", "cmd", "/K", "echo Typely CLI is ready. Try: typely-cli --help"])
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("osascript")
            .args(&["-e", "tell app \"Terminal\" to do script \"echo 'Typely CLI is ready. Try: typely-cli --help'\""])
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        // Try common terminal emulators
        let terminals = ["gnome-terminal", "konsole", "xterm", "alacritty", "terminator"];
        let mut terminal_opened = false;
        
        for terminal in &terminals {
            if Command::new("which").arg(terminal).output().map(|o| o.status.success()).unwrap_or(false) {
                Command::new(terminal)
                    .args(&["-e", "bash", "-c", "echo 'Typely CLI is ready. Try: typely-cli --help'; bash"])
                    .spawn()
                    .map_err(|e| format!("Failed to open terminal: {}", e))?;
                terminal_opened = true;
                break;
            }
        }
        
        if !terminal_opened {
            return Err("No suitable terminal emulator found".to_string());
        }
    }

    Ok("Terminal opened with CLI ready".to_string())
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
            check_cli_status,
            install_cli,
            uninstall_cli,
            open_terminal_with_cli,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}