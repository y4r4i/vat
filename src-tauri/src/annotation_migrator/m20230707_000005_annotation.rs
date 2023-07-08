use sea_orm::sea_query::Table;
use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use crate::annotation_migrator::m20230707_000003_image::Image;
use crate::annotation_migrator::m20230707_000004_assign::Assign;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230707_000005_annotation"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Annotation::Table)
                    .col(
                        ColumnDef::new(Annotation::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Annotation::AssignId).string().not_null())
                    .col(ColumnDef::new(Annotation::ImageId).string().not_null())
                    .col(ColumnDef::new(Annotation::Content).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Annotation::Table, Annotation::AssignId)
                            .to(Assign::Table, Assign::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Annotation::Table, Annotation::ImageId)
                            .to(Image::Table, Image::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Annotation::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Annotation {
    Table,
    Id,
    AssignId,
    ImageId,
    Content,
}
