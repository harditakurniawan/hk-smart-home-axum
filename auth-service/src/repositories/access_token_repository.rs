use std::time::Instant;

use entity::access_tokens::{ActiveModel, Entity as AccessTokens, Model, Column as AccessTokenColumn};
use migration::OnConflict;
use sea_orm::{ActiveValue::Set, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter};

use crate::utils::log_util::Log;

use super::user_repository::UserWithRole;

/**
 * use upset method
 */
pub async fn create_access_token(
    db: DatabaseConnection,
    user: UserWithRole,
    token: String,
) -> Result<InsertResult<entity::access_tokens::ActiveModel>, DbErr> {
    let start_time: Instant = Instant::now();

    let access_token = ActiveModel {
        user_id: Set(user.id),
        token: Set(token.clone()),
        ..Default::default()
    };

    return AccessTokens::insert(access_token)
        .on_conflict(
            OnConflict::column(AccessTokenColumn::UserId)
            .update_columns([
                AccessTokenColumn::Token,
                AccessTokenColumn::UserId,
                AccessTokenColumn::UpdatedAt,
            ])
            .to_owned()
        )
        .exec(&db)
        .await
        .map(|access_token| (access_token))
        .map_err(|err| {
            let duration: std::time::Duration = start_time.elapsed();

            if let Err(err) = Log::error(
                "DB".to_string(), 
                "create_access_token".to_string(), 
                serde_json::to_string(
                    &serde_json::json!({
                        "user_id": user.id,
                        "token": token,
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

pub async fn find_token(
    db: &DatabaseConnection,
    token: &String,
) -> Option<Model> {
    let start_time: Instant = Instant::now();

    return AccessTokens::find()
    .filter(AccessTokenColumn::Token.eq(token))
    .one(db)
    .await
    .map_err(|err| {
        let duration: std::time::Duration = start_time.elapsed();

        if let Err(err) = Log::error(
            "DB".to_string(), 
            "find_token".to_string(), 
            serde_json::to_string(
                &serde_json::json!({ "token": token })
            ).unwrap(), 
            err.to_string(), 
            duration.as_millis()
        ) {
            eprintln!("Failed to log error: {:?}", err);
        }

        err
    })
    .unwrap();
}

pub async fn delete_access_token(
    db: DatabaseConnection,
    user_id: i64,
) -> Result<(), DbErr> {
    let start_time: Instant = Instant::now();

    db.execute_unprepared(format!("DELETE FROM access_tokens WHERE user_id = {:?}", user_id).as_str())
    .await
    .map_err(|err| {
        let duration: std::time::Duration = start_time.elapsed();

        if let Err(err) = Log::error(
            "DB".to_string(), 
            "delete_access_token".to_string(), 
            serde_json::to_string(
                &serde_json::json!({
                    "user_id": user_id,
                })
            ).unwrap(), 
            err.to_string(), 
            duration.as_millis()
        ) {
            eprintln!("Failed to log error: {:?}", err);
        }

        err
    })?;

    Ok(())
}