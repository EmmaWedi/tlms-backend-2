use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    QueryFilter, Set,
};

use crate::{
    app::members::models::model::{AddMemberDto, UpdateMemberDto},
    apply_update_wrap, AppState,
};

pub async fn save_member(
    data: AddMemberDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::members::ActiveModel>, DbErr> {
    let member = entity::members::ActiveModel {
        first_name: Set(data.first_name),
        last_name: Set(data.last_name),
        email: Set(data.email),
        contact: Set(data.phone),
        residential_address: Set(data.address),
        gender: Set(data.gender),
        organization_id: Set(data.organization_id),
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

pub async fn get_all_members(
    state: &web::Data<AppState>,
) -> Result<Vec<entity::members::Model>, DbErr> {
    let members = entity::members::Entity::find()
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(members)
}

pub async fn get_member_by_id(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::members::Model, DbErr> {
    let member = entity::members::Entity::find_by_id(id)
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Organization not found or is blocked".into()));

    Ok(member.unwrap())
}

pub async fn get_member_by_phone(
    phone: &String,
    state: &web::Data<AppState>,
) -> Result<entity::members::Model, DbErr> {
    let member = entity::members::Entity::find()
        .filter(
            Condition::all()
                .add(entity::members::Column::Contact.eq(phone))
                .add(entity::members::Column::IsBlocked.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Organization not found or is blocked".into()));

    Ok(member.unwrap())
}

pub async fn toggle_member_blocked(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<(), DbErr> {
    let exists = entity::members::Entity::find_by_id(id)
        .one(state.pg_db.get_ref())
        .await?;

    let exists = match exists {
        Some(member) => member,
        None => return Err(DbErr::Custom("Member not found".to_string())),
    };

    let mut model: entity::members::ActiveModel = exists.into();

    if let ActiveValue::Set(is_blocked) = model.is_blocked {
        model.is_blocked = ActiveValue::Set(!is_blocked);
    }

    model.updated_at = ActiveValue::set(chrono::Utc::now().into());

    ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(())
}

pub async fn update_member(
    id: uuid::Uuid,
    member: UpdateMemberDto,
    state: &web::Data<AppState>,
) -> Result<entity::members::ActiveModel, DbErr> {
    let exists = entity::members::Entity::find_by_id(id)
        .filter(entity::members::Column::IsBlocked.eq(false))
        .one(state.pg_db.get_ref())
        .await?;

    let exists = match exists {
        Some(m) => m,
        None => return Err(DbErr::Custom("Member not found".to_string())),
    };

    let mut model: entity::members::ActiveModel = exists.into();

    apply_update_wrap!(model, member,
        first_name: first_name,
        last_name: last_name,
        email: email => Some,
        contact: phone,
        residential_address: address,
        gender: gender,
        department: department,
        aux_department: aux_department,
        sub_department: sub_department,
        member_type: member_type
    );

    model.updated_at = ActiveValue::set(chrono::Utc::now().into());

    let updated_member = ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(updated_member.into())
}
