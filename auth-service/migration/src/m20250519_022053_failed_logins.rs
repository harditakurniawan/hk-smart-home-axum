use sea_orm_migration::{prelude::*, schema::*};

const INDEX_NAME: &str = "IDX-FAILED_LOGINS-EMAIL";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FailedLogins::Table)
                    .if_not_exists()
                    .col(pk_auto(FailedLogins::Id))
                    .col(
                        ColumnDef::new(FailedLogins::Email)
                            .string()
                            .not_null()
                    )
                    .col(string(FailedLogins::Note))
                    .col(
                        ColumnDef::new(FailedLogins::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
            Index::create()
                .name(INDEX_NAME)
                .table(FailedLogins::Table)
                .col(FailedLogins::Email)
                .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name(INDEX_NAME).table(FailedLogins::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(FailedLogins::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum FailedLogins {
    Table,
    Id,
    Email,
    Note,
    CreatedAt,
}
