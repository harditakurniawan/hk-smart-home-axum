use dotenv::dotenv;

pub const PASSWORD_MIN_LENGTH: usize = 8;

#[derive(Clone)]
pub struct ApplicationConfig {
    pub app_name: String,
    pub app_prefix: String,
    pub app_port: String,
    pub app_cpu_multiplier: String,
    pub database_url: String,
    pub redis_url: String,
}

impl ApplicationConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| panic!("DB_HOST must be set."));
        let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| panic!("DB_NAME must be set."));
        let db_username = std::env::var("DB_USERNAME").unwrap_or_else(|_| panic!("DB_USERNAME must be set."));
        let db_password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| panic!("DB_PASSWORD must be set."));
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            eprintln!("DATABASE_URL is not set. Using default value.");
            format!("postgresql://{}:{}@{}/{}", db_username, db_password, db_host, db_name)
        });

        let redis_host = std::env::var("REDIS_HOST").unwrap_or_else(|_| panic!("REDIS_HOST must be set."));
        let redis_port = std::env::var("REDIS_PORT").unwrap_or_else(|_| panic!("REDIS_PORT must be set."));
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
            eprintln!("REDIS_URL is not set. Using default value.");
            format!("redis://{}:{}", redis_host, redis_port)
        });

        ApplicationConfig {
            app_name: std::env::var("APP_NAME").unwrap_or_else(|_| panic!("APP_NAME must be set.")),
            app_prefix: std::env::var("APP_PREFIX").unwrap_or_else(|_| panic!("APP_PREFIX must be set.")),
            app_port: std::env::var("APP_PORT").unwrap_or_else(|_| panic!("APP_PORT must be set.")),
            app_cpu_multiplier: std::env::var("APP_CPU_MULTIPLIER").unwrap_or_else(|_| panic!("APP_CPU_MULTIPLIER must be set.")),
            database_url,
            redis_url,
        }
    }
}