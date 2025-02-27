use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organization::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organization::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Organization::Name).string().not_null())
                    .col(
                        ColumnDef::new(Organization::Email)
                            .string()
                            .check(Expr::col(Organization::Email).like("%_@_%.__%")),
                    )
                    .col(
                        ColumnDef::new(Organization::Contact)
                            .string()
                            .not_null()
                            .unique_key()
                    )
                    .col(ColumnDef::new(Organization::Address).string().not_null())
                    .col(
                        ColumnDef::new(Organization::IsBlocked)
                            .boolean()
                            .not_null()
                            .default(Expr::val(false)),
                    )
                    .col(
                        ColumnDef::new(Organization::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Organization::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organization::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Organization {
    Table,
    Id,
    Name,
    Email,
    Contact,
    Address,
    IsBlocked,
    CreatedAt,
    UpdatedAt,
}
