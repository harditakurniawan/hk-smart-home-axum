use sea_orm_migration::{prelude::*, schema::*};

const INDEX_NAME: &str = "IDX-PERMISSIONS-PREFIX";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permissions::Table)
                    .if_not_exists()
                    .col(pk_auto(Permissions::Id))
                    .col(string(Permissions::Name))
                    .col(string(Permissions::Prefix))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
            Index::create()
                .name(INDEX_NAME)
                .table(Permissions::Table)
                .col(Permissions::Prefix)
                .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name(INDEX_NAME).table(Permissions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Permissions::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Permissions {
    Table,
    Id,
    Name,
    Prefix,
}
