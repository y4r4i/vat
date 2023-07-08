use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

mod m20230707_000000_schema;
mod m20230707_000001_catalog;
mod m20230707_000002_task;
mod m20230707_000003_image;
mod m20230707_000004_assign;
mod m20230707_000005_annotation;

pub struct AnnotationMigrator;

#[async_trait::async_trait]
impl MigratorTrait for AnnotationMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230707_000000_schema::Migration),
            Box::new(m20230707_000001_catalog::Migration),
            Box::new(m20230707_000002_task::Migration),
            Box::new(m20230707_000003_image::Migration),
            Box::new(m20230707_000004_assign::Migration),
            Box::new(m20230707_000005_annotation::Migration),
        ]
    }
}
