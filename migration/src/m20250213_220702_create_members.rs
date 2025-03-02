use sea_orm_migration::prelude::*;
use sea_orm::Statement;

use crate::m20250214_144741_create_organization::Organization;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE EXTENSION IF NOT EXISTS pgcrypto;".to_string(),
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Members::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Members::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Members::FirstName).string().not_null())
                    .col(ColumnDef::new(Members::LastName).string().not_null())
                    .col(
                        ColumnDef::new(Members::Email)
                            .string()
                            .check(Expr::col(Members::Email).like("%_@_%.__%")),
                    )
                    .col(
                        ColumnDef::new(Members::Contact)
                        .string()
                        .not_null()
                        .unique_key()
                    )
                    .col(
                        ColumnDef::new(Members::Gender).string().not_null().check(
                            Expr::col(Members::Gender).is_in(vec![
                                GenderEnum::Male.as_str(),
                                GenderEnum::Female.as_str(),
                            ]),
                        ),
                    )
                    .col(
                        ColumnDef::new(Members::DateOfBirth)
                            .date()
                            .not_null()
                            .check(Expr::col(Members::DateOfBirth).lte(Expr::cust("CURRENT_DATE")))
                            .check(Expr::col(Members::DateOfBirth).gte(Expr::value("1900-01-01"))),
                    )
                    .col(
                        ColumnDef::new(Members::ResidentialAddress)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Members::DateJoined)
                            .date()
                            .check(Expr::col(Members::DateJoined).lte(Expr::cust("CURRENT_DATE"))),
                    )
                    .col(
                        ColumnDef::new(Members::Department)
                            .string()
                            .not_null()
                            .check(Expr::col(Members::Department).is_in(vec![
                                DepartmentEnum::Youth.as_str(),
                                DepartmentEnum::Men.as_str(),
                                DepartmentEnum::Women.as_str(),
                                DepartmentEnum::Children.as_str(),
                                DepartmentEnum::NotSelected.as_str(),
                            ]))
                            .default(DepartmentEnum::NotSelected.as_str()),
                    )
                    .col(
                        ColumnDef::new(Members::AuxDepartment)
                            .string()
                            .not_null()
                            .check(Expr::col(Members::AuxDepartment).is_in(vec![
                                AuxDepartmentEnum::Pathfinders.as_str(),
                                AuxDepartmentEnum::YoungSingles.as_str(),
                                AuxDepartmentEnum::RoyalRangers.as_str(),
                                AuxDepartmentEnum::Missionettes.as_str(),
                                AuxDepartmentEnum::NotSelected.as_str(),
                            ]))
                            .default(AuxDepartmentEnum::NotSelected.as_str()),
                    )
                    .col(
                        ColumnDef::new(Members::SubDepartment)
                            .string()
                            .not_null()
                            .check(Expr::col(Members::SubDepartment).is_in(vec![
                                SubDepartmentEnum::Music.as_str(),
                                SubDepartmentEnum::Ushers.as_str(),
                                SubDepartmentEnum::Organizers.as_str(),
                                SubDepartmentEnum::NotSelected.as_str(),
                            ]))
                            .default(SubDepartmentEnum::NotSelected.as_str()),
                    )
                    .col(ColumnDef::new(Members::AddedBy).uuid())
                    .col(ColumnDef::new(Members::Alias).string())
                    .col(
                        ColumnDef::new(Members::MemberType)
                            .string()
                            .not_null()
                            .check(Expr::col(Members::MemberType).is_in(vec![
                                MemberTypeEnum::Member.as_str(),
                                MemberTypeEnum::Pastor.as_str(),
                                MemberTypeEnum::NotSelected.as_str(),
                            ]))
                            .default(MemberTypeEnum::NotSelected.as_str()),
                    )
                    .col(
                        ColumnDef::new(Members::IsBlocked)
                            .boolean()
                            .not_null()
                            .default(Expr::val(false)),
                    )
                    .col(ColumnDef::new(Members::OrganizationId).uuid().not_null())
                    .col(
                        ColumnDef::new(Members::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Members::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Members::Table, Members::OrganizationId)
                            .to(Organization::Table, Organization::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "DROP EXTENSION IF EXISTS pgcrypto;".to_string(),
            ))
            .await?;

        manager
            .drop_table(Table::drop().table(Members::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Members {
    Table,
    Id,
    FirstName,
    LastName,
    Contact,
    Gender,
    Email,
    DateOfBirth,
    ResidentialAddress,
    DateJoined,
    Department,
    AuxDepartment,
    SubDepartment,
    AddedBy,
    Alias,
    MemberType,
    IsBlocked,
    OrganizationId,
    CreatedAt,
    UpdatedAt,
}

enum DepartmentEnum {
    Men,
    Women,
    Youth,
    Children,
    NotSelected
}

enum MemberTypeEnum {
    Member,
    Pastor,
    NotSelected
}

enum AuxDepartmentEnum {
    Pathfinders,
    YoungSingles,
    RoyalRangers,
    Missionettes,
    NotSelected
}

enum SubDepartmentEnum {
    Music,
    Ushers,
    Organizers,
    NotSelected
}

enum GenderEnum {
    Male,
    Female,
}

impl AuxDepartmentEnum {
    pub fn as_str(&self) -> &str {
        match self {
            AuxDepartmentEnum::Pathfinders => "pathfinders",
            AuxDepartmentEnum::YoungSingles => "young_singles",
            AuxDepartmentEnum::RoyalRangers => "royal_rangers",
            AuxDepartmentEnum::Missionettes => "missionettes",
            AuxDepartmentEnum::NotSelected => "not_selected",
        }
    }
}

impl SubDepartmentEnum {
    pub fn as_str(&self) -> &str {
        match self {
            SubDepartmentEnum::Music => "music",
            SubDepartmentEnum::Ushers => "ushers",
            SubDepartmentEnum::Organizers => "organizers",
            SubDepartmentEnum::NotSelected => "not_selected",
        }
    }
}

impl GenderEnum {
    pub fn as_str(&self) -> &str {
        match self {
            GenderEnum::Male => "male",
            GenderEnum::Female => "female",
        }
    }
}

impl MemberTypeEnum {
    pub fn as_str(&self) -> &str {
        match self {
            MemberTypeEnum::Member => "member",
            MemberTypeEnum::Pastor => "pastor",
            MemberTypeEnum::NotSelected => "not_selected",
        }
    }
}

impl DepartmentEnum {
    pub fn as_str(&self) -> &str {
        match self {
            DepartmentEnum::Men => "men",
            DepartmentEnum::Women => "women",
            DepartmentEnum::Youth => "youth",
            DepartmentEnum::Children => "children",
            DepartmentEnum::NotSelected => "not_selected",
        }
    }
}
