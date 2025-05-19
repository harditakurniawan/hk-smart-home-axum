use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250420_030008_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AccessTokens::Table)
                    .if_not_exists()
                    .col(pk_auto(AccessTokens::Id))
                    .col(
                        ColumnDef::new(AccessTokens::Token)
                            .string()
                            .unique_key()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(AccessTokens::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AccessTokens::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AccessTokens::DeletedAt)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AccessTokens::UserId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AccessTokens::Table, AccessTokens::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccessTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AccessTokens {
    Table,
    Id,
    Token,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
