use sea_orm::sea_query::Table;
use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

use crate::migrator::m20230708_000000_destination::Destination;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230708_000001_history"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(History::Table)
                    .col(
                        ColumnDef::new(History::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(History::DestinationId).string().not_null())
                    .col(ColumnDef::new(History::AssignId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(History::Table, History::DestinationId)
                            .to(Destination::Table, Destination::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(History::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum History {
    Table,
    Id,
    DestinationId,
    AssignId,
}
