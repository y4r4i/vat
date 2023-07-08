use sea_orm::sea_query::Table;
use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use crate::annotation_migrator::m20230707_000002_task::Task;
use crate::annotation_migrator::m20230707_000003_image::Image;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230707_000004_assign"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Assign::Table)
                    .col(ColumnDef::new(Assign::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Assign::TaskId).string().not_null())
                    .col(ColumnDef::new(Assign::Worker).text().not_null())
                    .col(ColumnDef::new(Assign::LastWorkId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Assign::Table, Assign::TaskId)
                            .to(Task::Table, Task::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Assign::Table, Assign::LastWorkId)
                            .to(Image::Table, Image::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Assign::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Assign {
    Table,
    Id,
    TaskId,
    Worker,
    LastWorkId,
}
