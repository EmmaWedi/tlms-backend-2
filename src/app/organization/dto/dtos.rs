use actix_web::web;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, InsertResult,
    QueryFilter, Set,
};

use crate::{
    app::organization::models::model::{AddOrganizationDto, UpdateOrganizationDto},
    apply_update_wrap, AppState,
};

pub async fn save_organization(
    organization: AddOrganizationDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::organization::ActiveModel>, DbErr> {
    let data = entity::organization::ActiveModel {
        name: Set(organization.name),
        email: Set(Some(organization.email)),
        contact: Set(organization.phone),
        address: Set(organization.address),
        ..Default::default()
    };

    let insertion = entity::organization::Entity::insert(data)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database insert error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(insertion)
}

pub async fn get_organizations(
    state: &web::Data<AppState>,
) -> Result<Vec<entity::organization::Model>, DbErr> {
    let organizations = entity::organization::Entity::find()
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(organizations)
}

pub async fn get_organization_by_id(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<entity::organization::Model, DbErr> {
    let organization = entity::organization::Entity::find_by_id(id)
        .filter(entity::organization::Column::IsBlocked.eq(false))
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Organization not found or is blocked".into()));

    organization
}

pub async fn get_organization_by_email(
    email: &String,
    state: &web::Data<AppState>,
) -> Result<entity::organization::Model, DbErr> {
    let organization = entity::organization::Entity::find()
        .filter(
            Condition::all()
                .add(entity::organization::Column::Email.eq(email))
                .add(entity::organization::Column::IsBlocked.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Organization not found or is blocked".into()));

    organization
}

pub async fn get_organization_by_phone(
    phone: &String,
    state: &web::Data<AppState>,
) -> Result<entity::organization::Model, DbErr> {
    let organization = entity::organization::Entity::find()
        .filter(
            Condition::all()
                .add(entity::organization::Column::Contact.eq(phone))
                .add(entity::organization::Column::IsBlocked.eq(false)),
        )
        .one(state.pg_db.get_ref())
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Organization not found or is blocked".into()));

    organization
}

pub async fn update_organization(
    id: uuid::Uuid,
    organization: UpdateOrganizationDto,
    state: &web::Data<AppState>,
) -> Result<entity::organization::ActiveModel, DbErr> {
    let exists = entity::organization::Entity::find_by_id(id)
        .one(state.pg_db.get_ref())
        .await?;

    let exists = match exists {
        Some(m) => m,
        None => return Err(DbErr::Custom("Organization not found".to_string())),
    };

    let mut model: entity::organization::ActiveModel = exists.into();

    apply_update_wrap!(model, organization,
        name: name,
        email: email => Some,
        contact: phone,
        address: address
    );

    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    let updated_org = ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(updated_org.into())
}

pub async fn toggle_block(id: uuid::Uuid, state: &web::Data<AppState>) -> Result<(), DbErr> {
    let exists = entity::organization::Entity::find_by_id(id)
        .one(state.pg_db.get_ref())
        .await?;

    let exists = match exists {
        Some(m) => m,
        None => return Err(DbErr::Custom("Organization not found".to_string())),
    };

    let mut model: entity::organization::ActiveModel = exists.into();

    if let ActiveValue::Set(is_blocked) = model.is_blocked {
        model.is_blocked = ActiveValue::Set(!is_blocked);
    }
    model.updated_at = ActiveValue::Set(chrono::Utc::now().into());

    ActiveModelTrait::update(model, state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database update error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(())
}
