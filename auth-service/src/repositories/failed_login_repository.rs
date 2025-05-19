use std::time::Instant;

use entity::failed_logins;
use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

use crate::utils::log_util::Log;

pub async fn create_failed_login(
    db: &DatabaseConnection,
    email: &String,
    note: Option<&str>
) -> Result<(), DbErr> {
    let start_time: Instant = Instant::now();

    let failed_login = failed_logins::ActiveModel {
        email: Set(email.clone()),
        note: Set(note.unwrap_or_default().to_string()),
        ..Default::default()
    };
    
    return failed_logins::Entity::insert(failed_login).exec(db).await
        .map(|_| ())
        .map_err(|err| {
            let duration: std::time::Duration = start_time.elapsed();

            if let Err(err) = Log::error(
                "DB".to_string(), 
                "create_failed_login".to_string(), 
                serde_json::to_string(
                    &serde_json::json!({
                        "email": email,
                        "note": note.unwrap_or_default(),
                    })
                ).unwrap(), 
                err.to_string(), 
                duration.as_millis()
            ) {
                eprintln!("Failed to log error: {:?}", err);
            }

            err
        });
}