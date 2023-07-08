use sea_orm::sea_query::Table;
use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230707_000000_schema"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Schema::Table)
                    .col(ColumnDef::new(Schema::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Schema::Content).text().not_null())
                    .col(ColumnDef::new(Schema::Hash).text().not_null().unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Schema::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Schema {
    Table,
    Id,
    Content,
    Hash,
}
