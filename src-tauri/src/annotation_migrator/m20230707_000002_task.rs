use sea_orm::sea_query::Table;
use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use crate::annotation_migrator::m20230707_000000_schema::Schema;
use crate::annotation_migrator::m20230707_000001_catalog::Catalog;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230707_000002_task"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .col(ColumnDef::new(Task::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Task::SchemaId).string().not_null())
                    .col(ColumnDef::new(Task::CatalogId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Task::Table, Task::SchemaId)
                            .to(Schema::Table, Schema::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Task::Table, Task::SchemaId)
                            .to(Catalog::Table, Catalog::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Task {
    Table,
    Id,
    SchemaId,
    CatalogId,
}
