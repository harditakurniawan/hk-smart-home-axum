use std::time::Instant;

use entity::{roles, user_roles, users};
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter};

use crate::utils::log_util::Log;

// pub async fn find_role(db: &DatabaseConnection, role: &str) -> Result<roles::Model, DbErr> {
//     match roles::Entity::find()
//         .filter(roles::Column::Name.eq(role))
//         .one(db).await? {
//             Some(role_model) => Ok(role_model),
//         None => Err(DbErr::Custom("Role not found".to_string())),
//     }
// }

pub async fn set_role_to_user(db: &DatabaseTransaction, user: users::Model, role: &str) -> Result<(), DbErr> {
    let start_time: Instant = Instant::now();

    let role = match roles::Entity::find().filter(roles::Column::Name.eq(role)).one(db).await? {
        Some(record) => record,
        None => return Err(DbErr::Custom("Role not found".to_string())),
    };

    let user_role_model = user_roles::ActiveModel {
        user_id: Set(user.id),
        role_id: Set(role.id),
        ..Default::default()
    };

    return user_roles::Entity::insert(user_role_model).exec(db)
    .await
    .map(|_| ())
    .map_err(|err| {
        let duration: std::time::Duration = start_time.elapsed();

        if let Err(err) = Log::error(
            "DB".to_string(), 
            "set_role_to_user".to_string(), 
            serde_json::to_string(
                &serde_json::json!({
                    "user_id": user.id,
                    "role_id": role.id,
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