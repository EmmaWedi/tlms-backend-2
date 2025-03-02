//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    #[sea_orm(unique)]
    pub contact: String,
    pub gender: String,
    pub date_of_birth: Date,
    pub residential_address: String,
    pub date_joined: Option<Date>,
    pub department: String,
    pub aux_department: String,
    pub sub_department: String,
    pub added_by: Option<Uuid>,
    pub alias: Option<String>,
    pub member_type: String,
    pub is_blocked: bool,
    pub organization_id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrganizationId",
        to = "super::organization::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Organization,
    #[sea_orm(has_many = "super::users::Entity")]
    Users,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
