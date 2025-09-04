use anyhow::Result;
use log::info;
use std::sync::Arc;
use typely::app::services::TypelyService;
use typely::infra::database::DatabaseConnection;
use typely::infra::engine::TextExpansionEngine;
use typely::infra::get_default_database_path;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let db_path = get_default_database_path()?;
    info!("Using database: {}", db_path.display());

    let db_connection = DatabaseConnection::new(&db_path).await?;
    let service = TypelyService::new(db_connection).await;

    info!("Starting Typely text expansion engine...");
    let engine = TextExpansionEngine::new(Arc::new(service), None)?;
    engine.start().await?;

    Ok(())
}
