use sea_orm::sea_query::Table;
use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230707_000001_catalog"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Catalog::Table)
                    .col(
                        ColumnDef::new(Catalog::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Catalog::Name).text().not_null())
                    .col(ColumnDef::new(Catalog::Hash).text().not_null().unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Catalog::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Catalog {
    Table,
    Id,
    Name,
    Hash,
}
