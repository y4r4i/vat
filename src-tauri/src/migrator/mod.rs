use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

mod m20230708_000000_destination;
mod m20230708_000001_history;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230708_000000_destination::Migration),
            Box::new(m20230708_000001_history::Migration),
        ]
    }
}
