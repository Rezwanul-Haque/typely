use std::process::Command;
use std::fs;
use tempfile::TempDir;
use serde_json::Value;

#[test]
fn test_cli_add_and_list_snippets() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Add a snippet
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "add", "::test", "Test content"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success(), "Failed to add snippet: {}", String::from_utf8_lossy(&output.stderr));
    
    // List snippets
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "list"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success(), "Failed to list snippets: {}", String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("::test"));
    assert!(stdout.contains("Test content"));
}

#[test]
fn test_cli_export_import() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let export_path = temp_dir.path().join("export.json");
    
    // Add some test snippets
    let test_snippets = [
        ("::hello", "Hello, World!"),
        ("::bye", "Goodbye!"),
    ];
    
    for (trigger, replacement) in &test_snippets {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "typely-cli", "--", 
                    "--database", db_path.to_str().unwrap(),
                    "add", trigger, replacement])
            .output()
            .expect("Failed to run typely-cli");
        
        assert!(output.status.success());
    }
    
    // Export snippets
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "export", export_path.to_str().unwrap()])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success(), "Failed to export: {}", String::from_utf8_lossy(&output.stderr));
    
    // Verify export file was created and contains correct data
    let export_content = fs::read_to_string(&export_path).expect("Failed to read export file");
    let json: Value = serde_json::from_str(&export_content).expect("Invalid JSON");
    
    assert!(json.is_array());
    let snippets = json.as_array().unwrap();
    assert_eq!(snippets.len(), 2);
    
    // Create new database and import
    let new_db_path = temp_dir.path().join("new_test.db");
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", new_db_path.to_str().unwrap(),
                "import", export_path.to_str().unwrap()])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success(), "Failed to import: {}", String::from_utf8_lossy(&output.stderr));
    
    // Verify import worked
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", new_db_path.to_str().unwrap(),
                "list"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("::hello"));
    assert!(stdout.contains("::bye"));
}

#[test]
fn test_cli_expand() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Add a snippet with dynamic placeholder
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "add", "::today", "Today is {date}"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success());
    
    // Test expansion
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "expand", "::today"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success(), "Failed to expand: {}", String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Today is"));
    // Should contain a date in YYYY-MM-DD format
    assert!(stdout.contains(chrono::Utc::now().format("%Y").to_string().as_str()));
}

#[test]
fn test_cli_stats() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Add some snippets
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "add", "::test", "Test content"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success());
    
    // Get stats
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "stats"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success(), "Failed to get stats: {}", String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Total snippets: 1"));
}

#[test]
fn test_cli_tags() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Add snippets with tags
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "add", "::work", "Work content", "--tags", "work,business"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success());
    
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "add", "::personal", "Personal content", "--tags", "personal"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success());
    
    // List with tag filter
    let output = Command::new("cargo")
        .args(&["run", "--bin", "typely-cli", "--", 
                "--database", db_path.to_str().unwrap(),
                "list", "--tags", "work"])
        .output()
        .expect("Failed to run typely-cli");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("::work"));
    assert!(!stdout.contains("::personal"));
}