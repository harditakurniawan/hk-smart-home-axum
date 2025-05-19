use std::time::Instant;

use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait};
use entity::{roles, user_roles, users};

use crate::utils::log_util::Log;

#[derive(Debug, FromQueryResult, Clone)]
pub struct UserWithRole {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub role_name: String,
}

pub async fn find_user_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Option<users::Model> {
    let start_time: Instant = Instant::now();

    return users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .map(|record: Option<users::Model>| (record))
        .map_err(|err: DbErr| {
            let duration: std::time::Duration = start_time.elapsed();

            if let Err(err) = Log::error(
                "DB".to_string(), 
                "find_user_by_email".to_string(), 
                serde_json::to_string(
                    &serde_json::json!({ "email": email })
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

pub async fn create_user(
    db: &DatabaseTransaction,
    name: String,
    email: String,
    password: String,
) -> Result<users::Model, DbErr> {
    let start_time: Instant = Instant::now();

    let user = users::ActiveModel {
        name: Set(name.clone()),
        email: Set(email.clone()),
        password: Set(password.clone()),
        ..Default::default()
    };
    
    return users::Entity::insert(user).exec_with_returning(db).await
        .map_err(|err: DbErr| {
            let duration: std::time::Duration = start_time.elapsed();

            if let Err(err) = Log::error(
                "DB".to_string(), 
                "create_user".to_string(), 
                serde_json::to_string(
                    &serde_json::json!({ 
                        "name": name,
                        "email": email,
                        "password": password,
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

pub async fn find_user_by_email_with_roles(
    db: &DatabaseConnection,
    email: &String
) -> Result<Option<UserWithRole>, DbErr> {
    let start_time: Instant = Instant::now();

    return users::Entity::find()
        .column_as(roles::Column::Name, "role_name")
        .join_rev(JoinType::LeftJoin, user_roles::Relation::Users.def())
        .join(JoinType::LeftJoin, user_roles::Relation::Roles.def())
        .filter(users::Column::Email.eq(email))
        .into_model::<UserWithRole>()
        .one(db)
        .await
        .map(|record| (record))
        .map_err(|err| {
            let duration: std::time::Duration = start_time.elapsed();

            if let Err(err) = Log::error(
                "DB".to_string(), 
                "find_user_by_email_with_roles".to_string(), 
                serde_json::to_string(
                    &serde_json::json!({ "email": email })
                ).unwrap(), 
                err.to_string(), 
                duration.as_millis()
            ) {
                eprintln!("Failed to log error: {:?}", err);
            }

            err
        });
}