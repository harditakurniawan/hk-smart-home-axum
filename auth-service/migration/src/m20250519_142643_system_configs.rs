use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemConfigs::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemConfigs::Id))
                    .col(string(SystemConfigs::Key))
                    .col(string(SystemConfigs::Value))
                    .col(string(SystemConfigs::Note))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemConfigs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SystemConfigs {
    Table,
    Id,
    Key,
    Value,
    Note,
}
