use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::error::Error;

pub async fn setup_database(database_url: &str) -> Result<DatabaseConnection, Box<dyn Error>> {
    let mut options = ConnectOptions::new(database_url.to_string());
    options
        .max_connections(20)
        .min_connections(5)
        .sqlx_logging(false);
    
    Database::connect(options)
        .await
        .map_err(|e| format!("Database connection failed: {}", e).into())
}