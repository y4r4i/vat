use sea_orm::sea_query::Table;
use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use crate::annotation_migrator::m20230707_000001_catalog::Catalog;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230707_000003_image"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .col(ColumnDef::new(Image::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Image::CatalogId).string().not_null())
                    .col(ColumnDef::new(Image::Content).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Image::Table, Image::CatalogId)
                            .to(Catalog::Table, Catalog::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Image::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Image {
    Table,
    Id,
    CatalogId,
    Content,
}
