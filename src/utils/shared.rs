use actix_web::web;
use sea_orm::{ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, QueryFilter, Set};

use crate::AppState;

use super::models::SaveMemberOrgDto;

pub async fn save_member_from_org(
    id: uuid::Uuid,
    data: SaveMemberOrgDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::members::ActiveModel>, DbErr> {
    let exists = entity::members::Entity::find()
        .filter(
            Condition::all()
                .add(entity::members::Column::IsBlocked.eq(false))
                .add(entity::members::Column::Contact.eq(&data.phone))
                .add(entity::members::Column::OrganizationId.eq(id)),
        )
        .one(state.pg_db.get_ref())
        .await?;

    if let Some(_) = exists {
        return Err(DbErr::Custom("Member already exists".to_string()));
    }

    let member = entity::members::ActiveModel {
        first_name: Set(data.first_name),
        last_name: Set(data.last_name),
        email: Set(data.email),
        contact: Set(data.phone),
        residential_address: Set(data.address),
        organization_id: Set(id),
        gender: Set(data.gender),
        date_joined: Set(data.date_joined),
        ..Default::default()
    };

    let insertion = entity::members::Entity::insert(member)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(insertion)
}
