//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub destination_id: String,
    pub assign_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::destination::Entity",
        from = "Column::DestinationId",
        to = "super::destination::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Destination,
}

impl Related<super::destination::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Destination.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}