use sea_orm_migration::{prelude::*, schema::*};

const INDEX_NAME: &str = "IDX-ATTEMPT_LOGINS-EMAIL";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AttemptLogins::Table)
                    .if_not_exists()
                    .col(pk_auto(AttemptLogins::Id))
                    .col(string(AttemptLogins::Email))
                    .col(ColumnDef::new(AttemptLogins::Payload).text())
                    .col(
                        ColumnDef::new(AttemptLogins::CreatedAt)
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
                .table(AttemptLogins::Table)
                .col(AttemptLogins::Email)
                .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name(INDEX_NAME).table(AttemptLogins::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AttemptLogins::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum AttemptLogins {
    Table,
    Id,
    Email,
    Payload,
    CreatedAt,
}
