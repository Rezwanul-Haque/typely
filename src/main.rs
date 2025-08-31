use typely_lib::application::TypelyService;
use typely_lib::infrastructure::{DatabaseConnection, get_default_database_path};
use typely_lib::presentation::TextExpansionEngine;
use anyhow::Result;
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    log::info!("Starting Typely text expansion application");

    // Initialize database connection
    let db_path = get_default_database_path()?;
    log::info!("Using database: {}", db_path.display());
    
    let db_connection = DatabaseConnection::new(&db_path).await?;
    
    // Create service
    let service = Arc::new(TypelyService::new(db_connection).await);
    
    // Add sample snippets in debug mode
    #[cfg(debug_assertions)]
    {
        if let Err(e) = add_sample_snippets(&service).await {
            log::warn!("Failed to add sample snippets: {}", e);
        }
    }
    
    // Create and start text expansion engine
    let engine = TextExpansionEngine::new(service, None)?;
    
    log::info!("Starting text expansion engine...");
    engine.start().await?;
    
    // Wait for shutdown signal
    log::info!("Typely is running. Press Ctrl+C to stop.");
    
    match signal::ctrl_c().await {
        Ok(()) => {
            log::info!("Shutdown signal received");
        }
        Err(err) => {
            log::error!("Unable to listen for shutdown signal: {}", err);
        }
    }
    
    // Stop the engine
    engine.stop();
    log::info!("Typely stopped");
    
    Ok(())
}

// Add some sample snippets for testing
#[cfg(debug_assertions)]
async fn add_sample_snippets(service: &TypelyService) -> Result<()> {
    use typely_lib::application::dto::CreateSnippetRequest;
    
    let samples = vec![
        ("::hello", "Hello, World!"),
        ("::email", "your.email@example.com"),
        ("::phone", "+1 (555) 123-4567"),
        ("::addr", "123 Main St, Anytown, ST 12345"),
        ("::sig", "Best regards,\nYour Name"),
        ("::date", "{date}"),
        ("::time", "{time}"),
        ("::brb", "Be right back!"),
        ("::asap", "As soon as possible"),
        ("::ty", "Thank you!"),
    ];
    
    for (trigger, replacement) in samples {
        // Check if snippet already exists
        if service.get_snippet_by_trigger(trigger).await?.is_none() {
            let request = CreateSnippetRequest {
                trigger: trigger.to_string(),
                replacement: replacement.to_string(),
                tags: Some(vec!["sample".to_string()]),
            };
            
            match service.create_snippet(request).await {
                Ok(_) => log::debug!("Created sample snippet: {}", trigger),
                Err(e) => log::warn!("Failed to create sample snippet {}: {}", trigger, e),
            }
        }
    }
    
    Ok(())
}
