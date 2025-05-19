use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250420_030008_users::Users;

const INDEX_NAME: &str = "IDX-BANNED_LOGINS-EMAIL";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BannedLogins::Table)
                    .if_not_exists()
                    .col(pk_auto(BannedLogins::Id))
                    .col(
                        ColumnDef::new(BannedLogins::Email)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(BannedLogins::BannedUntil)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(BannedLogins::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_banned_logins_email")
                            .from(BannedLogins::Table, BannedLogins::Email)
                            .to(Users::Table, Users::Email)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
            Index::create()
                .name(INDEX_NAME)
                .table(BannedLogins::Table)
                .col(BannedLogins::Email)
                .to_owned(),
            )
            .await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name(INDEX_NAME).table(BannedLogins::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(BannedLogins::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum BannedLogins {
    Table,
    Id,
    Email,
    BannedUntil,
    CreatedAt,
}
