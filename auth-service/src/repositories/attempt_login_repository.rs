use std::time::Instant;

use axum::Json;
use entity::attempt_logins;
use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};

use crate::{core::dtos::auth::sign_in_dto::SignInDto, utils::log_util::Log};

pub async fn create_attempt_login(
    db: &DatabaseConnection,
    sign_in_dto: Json<SignInDto>
) -> Result<(), DbErr> {
    let start_time: Instant = Instant::now();

    let attempt_login = attempt_logins::ActiveModel {
        email: Set(sign_in_dto.email.clone()),
        payload: Set(Some(serde_json::to_string(&sign_in_dto.0).unwrap())),
        ..Default::default()
    };
    
    return attempt_logins::Entity::insert(attempt_login).exec(db).await
        .map(|_| ())
        .map_err(|err| {
            let duration: std::time::Duration = start_time.elapsed();

            if let Err(err) = Log::error(
                "DB".to_string(), 
                "create_attempt_login".to_string(), 
                serde_json::to_string(
                    &serde_json::json!({
                        "email": sign_in_dto.email,
                        "payload": serde_json::to_string(&sign_in_dto.0).unwrap(),
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