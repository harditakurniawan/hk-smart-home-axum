use entity::system_configs;
use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue::Set, EntityTrait, PaginatorTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db: &SchemaManagerConnection<'_> = manager.get_connection();

        let is_system_configs_exists = system_configs::Entity::find().count(db).await?;

        if is_system_configs_exists > 0 {
            println!("seed_create_system_config already generated");
            return Ok(());
        }

        let system_config: Vec<system_configs::ActiveModel> = vec![
            system_configs::ActiveModel {
                key: Set("max_retry_login".to_string()),
                value: Set("3".to_string()),
                note: Set("unit".to_string()),
                ..Default::default()
            },
            system_configs::ActiveModel {
                key: Set("banned_time_login".to_string()),
                value: Set("15".to_string()),
                note: Set("in minutes".to_string()),
                ..Default::default()
            },
        ];

        if let Err(err) = system_configs::Entity::insert_many(system_config).exec(db).await {
            eprintln!("Failed to insert seed_create_system_config | roles: {:?}", err);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
