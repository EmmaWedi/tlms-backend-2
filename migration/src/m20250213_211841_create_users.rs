use sea_orm_migration::prelude::*;

use crate::m20250213_220702_create_members::Members;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .check(Expr::col(Users::Email).like("%_@_%.__%")),
                    )
                    .col(ColumnDef::new(Users::MemberId).uuid().not_null())
                    .col(ColumnDef::new(Users::SessionId).uuid())
                    .col(
                        ColumnDef::new(Users::Contact)
                        .string()
                        .not_null()
                        .unique_key()
                    )
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::Role).string().not_null())
                    .col(
                        ColumnDef::new(Users::IsPasswordChanged)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Users::Table, Users::MemberId)
                            .to(Members::Table, Members::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Email,
    Contact,
    Password,
    Role,
    IsPasswordChanged,
    MemberId,
    SessionId,
    CreatedAt,
    UpdatedAt,
}
