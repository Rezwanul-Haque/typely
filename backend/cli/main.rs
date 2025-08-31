use typely::app::services::TypelyService;
use typely::infra::{DatabaseConnection, get_default_database_path};
use typely::cli::{TypelyArgs, TypelyCliHandler};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let args = TypelyArgs::parse();

    // Initialize logging based on verbosity
    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }

    // Determine database path
    let db_path = if let Some(db_path) = args.database {
        PathBuf::from(db_path)
    } else {
        get_default_database_path()?
    };

    if args.verbose {
        eprintln!("Using database: {}", db_path.display());
    }

    // Initialize database connection
    let db_connection = DatabaseConnection::new(&db_path).await?;

    // Create service
    let service = TypelyService::new(db_connection).await;

    // Create CLI handler
    let handler = TypelyCliHandler::new(service);

    // Handle command
    handler.handle_command(args.command, args.verbose).await?;

    Ok(())
}