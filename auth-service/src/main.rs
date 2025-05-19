mod config;
mod routes;
mod core;
mod controllers;
mod utils;
mod middlewares;
mod services;
mod repositories;
mod databases;

use std::borrow::Cow;
use std::env;
use axum::middleware::from_fn;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use databases::postgre::connection::setup_database;
use databases::redis::connection::setup_redis;
use dotenv::dotenv;

use axum::Extension;
use axum::{
    http::StatusCode, response::IntoResponse, Router
};
use middlewares::log_middleware::logging_middleware;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use config::application_config::ApplicationConfig;
use routes::v1::auth_route::AUTH as auth_router;
use routes::v1::profile_route::PROFILE as profile_router;
use tokio::runtime::Builder;
use utils::response_util::GlobalResponse;
use std::error::Error;
use validator::ValidationErrors;

#[derive(Clone, Debug)]
pub struct JwtKeys {
    public_key: String,
    private_key: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load config
    let config = load_config()?;
    let jwt_keys = load_jwt_keys()?;

    // Create custom runtime
    let runtime = create_runtime(&config)?;

    // Run async main
    runtime.block_on(async {
        let db: DatabaseConnection = setup_database(&config.database_url).await?;
        let redis: Pool<RedisConnectionManager> = setup_redis(&config.redis_url).await;
        let app = configure_routes(&config, db, redis, jwt_keys)?;
        run_server(config, app).await?;
        Ok::<(), Box<dyn Error>>(())
    })?;
    Ok(())
}

fn load_config() -> Result<ApplicationConfig, Box<dyn Error>> {
    dotenv().ok();
    return Ok(ApplicationConfig::new()).map_err(|e: Box<dyn Error>| format!("Failed to load application config: {}", e).into());
}

fn load_jwt_keys() -> Result<JwtKeys, Box<dyn Error>> {
    let current_dir = env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    let private_key_path = current_dir.join("src/config/jwt-private.key");
    let public_key_path = current_dir.join("src/config/jwt-public.key");

    let private_key = std::fs::read_to_string(&private_key_path).map_err(|e| format!("Failed to read JWT private key: {}", e))?;
    let public_key = std::fs::read_to_string(&public_key_path).map_err(|e| format!("Failed to read JWT public key: {}", e))?;

    if private_key.is_empty() || public_key.is_empty() {
        return Err("JWT keys are empty".into());
    }

    Ok(JwtKeys { public_key, private_key })
}

fn create_runtime(config: &ApplicationConfig) -> Result<tokio::runtime::Runtime, Box<dyn Error>> {
    let cpu_multiplier = config.app_cpu_multiplier.parse::<usize>()?;
    let num_cpus = num_cpus::get();
    println!("Number of CPU {:?} and CPU multiplier {:?}", &num_cpus, &cpu_multiplier);
    
    Builder::new_multi_thread()
    .worker_threads(num_cpus * cpu_multiplier)
    .enable_all()
    .build()
    .map_err(|e| format!("Failed to create Tokio runtime: {}", e).into())
}

fn configure_routes(
    config: &ApplicationConfig,
    db: DatabaseConnection,
    redis: Pool<RedisConnectionManager>,
    jwt_keys: JwtKeys,
) -> Result<Router, Box<dyn Error>> {
    let v1 = Router::new()
                        .nest("/v1", auth_router.clone().merge(profile_router.clone()));
    
    Ok(
        Router::new()
        .nest(&format!("/{}", config.app_prefix), v1)
        .layer(Extension(db))
        .layer(Extension(redis))
        .layer(Extension(config.clone()))
        .layer(Extension(jwt_keys))
        .layer(from_fn(logging_middleware))
        .fallback(handle_404)
        .method_not_allowed_fallback(handle_405)
    )
}

async fn handle_404() -> impl IntoResponse {
    return (
        GlobalResponse::<()>::error(StatusCode::NOT_FOUND, StatusCode::NOT_FOUND.to_string())
            .with_validation_errors({
                let mut errors = ValidationErrors::new();
                let mut validation_error = validator::ValidationError::new(StatusCode::NOT_FOUND.as_str());
                validation_error.message = Some(Cow::Owned(StatusCode::NOT_FOUND.to_string()));
                errors.add(StatusCode::NOT_FOUND.as_str(), validation_error);
                errors
            })
    ).into_response();
}

async fn handle_405() -> impl IntoResponse {
    return (
        GlobalResponse::<()>::error(StatusCode::METHOD_NOT_ALLOWED, StatusCode::METHOD_NOT_ALLOWED.to_string())
            .with_validation_errors({
                let mut errors = ValidationErrors::new();
                let mut validation_error = validator::ValidationError::new(StatusCode::METHOD_NOT_ALLOWED.as_str());
                validation_error.message = Some(Cow::Owned(StatusCode::METHOD_NOT_ALLOWED.to_string()));
                errors.add(StatusCode::METHOD_NOT_ALLOWED.as_str(), validation_error);
                errors
            })
    ).into_response();
}

async fn run_server(config: ApplicationConfig, app: Router) -> Result<(), Box<dyn Error>> {
    let bind_address: String = format!("0.0.0.0:{}", config.app_port);
    let listener: TcpListener = TcpListener::bind(&bind_address).await.map_err(|e| format!("Failed to bind to {}: {}", bind_address, e))?;
    
    println!("{} started on port {} 🚀!", config.app_name, config.app_port);
    
    axum::serve(listener, app).await
        .map_err(|e| format!("Server error: {}", e).into())
}