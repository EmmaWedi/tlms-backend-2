use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Media::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Media::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Media::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Media::FileName).string())
                    .col(ColumnDef::new(Media::FilePath).string())
                    .col(ColumnDef::new(Media::MimeType).string())
                    .col(ColumnDef::new(Media::FileSize).big_integer())
                    .col(ColumnDef::new(Media::MediaType).string())
                    .col(ColumnDef::new(Media::Width).integer())
                    .col(ColumnDef::new(Media::Height).integer())
                    .col(ColumnDef::new(Media::Duration).integer())
                    .col(
                        ColumnDef::new(Media::IsDeleted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Media::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Media::UpdatedAt)
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
            .drop_table(Table::drop().table(Media::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Media {
    Table,
    Id,
    OwnerId,
    FileName,
    FilePath,
    MimeType,
    FileSize,
    MediaType,
    Width,
    Height,
    Duration,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}
