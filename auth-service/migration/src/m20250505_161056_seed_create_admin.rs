use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter}};
use bcrypt::{hash, DEFAULT_COST};
use entity::{roles, user_roles, users};

use crate::m20250505_061041_seed_role_permission::ADMIN_ROLE;

#[derive(DeriveMigrationName)]
pub struct Migration;

pub const ADMIN_EMAIL: &str = "admin@hksmarthome.com";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db: &SchemaManagerConnection<'_> = manager.get_connection();

        let is_admin_exists = find_admin(db).await;

        if is_admin_exists.is_some() {
            println!("seed_create_admin already generated");
            return  Ok(());
        }

        let admin = create_admin(db).await?;
        set_role_to_admin(db, admin).await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

async fn find_admin(db: &SchemaManagerConnection<'_>) -> Option<users::Model> {
    users::Entity::find()
        .filter(users::Column::Email.eq(ADMIN_EMAIL))
        .one(db)
        .await
        .unwrap_or(None)
}

async fn find_admin_role(db: &SchemaManagerConnection<'_>) -> Option<roles::Model> {
    roles::Entity::find()
        .filter(roles::Column::Name.eq("admin"))
        .one(db)
        .await
        .unwrap_or(None)
}

async fn create_admin(db: &SchemaManagerConnection<'_>) -> Result<users::Model, DbErr> {
    let admin = users::ActiveModel {
        name: Set(ADMIN_ROLE.to_string().to_uppercase()),
        email: Set(ADMIN_EMAIL.to_string()),
        password: Set(hash("admin#2025!", DEFAULT_COST).unwrap()),
        ..Default::default()
    };

    let inserted_user = users::Entity::insert(admin)
        .exec_with_returning(db)
        .await
        .map_err(|err| {
            eprintln!("Failed to insert seed_create_admin | create admin: {:?}", err);
            err
        })?;

    Ok(inserted_user)
}

async fn set_role_to_admin(db: &SchemaManagerConnection<'_>, admin: users::Model) {
    let admin_role = find_admin_role(db).await;

    let admin_role = match admin_role {
        Some(record) => record,
        None => return,
    };

    let user_role_model = user_roles::ActiveModel {
        user_id: Set(admin.id),
        role_id: Set(admin_role.id),
        ..Default::default()
    };

    if let Err(err) = user_roles::Entity::insert(user_role_model).exec(db).await {
        eprintln!("Failed to insert seed_create_admin | create admin role: {:?}", err);
    }
}
