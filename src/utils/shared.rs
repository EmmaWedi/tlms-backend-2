use actix_web::web;
use sea_orm::{ColumnTrait, Condition, DbErr, EntityTrait, InsertResult, QueryFilter, Set};

use crate::AppState;

use super::{
    file_methods::file_exists,
    models::{SaveMediaDto, SaveMemberOrgDto},
};

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

pub async fn save_media_meta(
    owner: uuid::Uuid,
    data: SaveMediaDto,
    state: &web::Data<AppState>,
) -> Result<InsertResult<entity::media::ActiveModel>, DbErr> {
    let extension = data.mime_type.split('/').nth(1);

    if !file_exists(&data.file_name, &extension.unwrap()).await {
        return Err(DbErr::Custom("File does not exist".to_string()));
    };

    let media_data = entity::media::ActiveModel {
        owner_id: Set(owner),
        file_path: Set(Some(data.file_path)),
        mime_type: Set(Some(data.mime_type)),
        file_size: Set(Some(data.file_size)),
        file_name: Set(Some(data.file_name)),
        media_type: Set(Some(data.media_type)),
        width: Set(data.width),
        height: Set(data.height),
        duration: Set(data.duration),
        ..Default::default()
    };

    let insertion = entity::media::Entity::insert(media_data)
        .exec(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(insertion)
}

//get media by id
pub async fn get_media_by_id(
    id: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Option<entity::media::Model>, DbErr> {
    let media = entity::media::Entity::find_by_id(id)
        .one(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(media)
}

//get media by user
pub async fn get_media_by_user(
    owner: uuid::Uuid,
    state: &web::Data<AppState>,
) -> Result<Vec<entity::media::Model>, DbErr> {
    let medias = entity::media::Entity::find()
        .filter(Condition::all().add(entity::media::Column::OwnerId.eq(owner)))
        .all(state.pg_db.get_ref())
        .await
        .map_err(|err| {
            eprintln!("Database retrieval error: {}", err);
            DbErr::Custom(err.to_string())
        })?;

    Ok(medias)
}
