pub use sea_orm_migration::prelude::*;

mod m20250213_211841_create_users;
mod m20250213_220702_create_members;
mod m20250214_144741_create_organization;
mod m20250214_150448_create_media_schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250214_144741_create_organization::Migration),
            Box::new(m20250213_220702_create_members::Migration),
            Box::new(m20250213_211841_create_users::Migration),
            Box::new(m20250214_150448_create_media_schema::Migration),
        ]
    }
}
