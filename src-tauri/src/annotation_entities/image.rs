//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "image")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub catalog_id: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::annotation::Entity")]
    Annotation,
    #[sea_orm(has_many = "super::assign::Entity")]
    Assign,
    #[sea_orm(
        belongs_to = "super::catalog::Entity",
        from = "Column::CatalogId",
        to = "super::catalog::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Catalog,
}

impl Related<super::annotation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Annotation.def()
    }
}

impl Related<super::assign::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Assign.def()
    }
}

impl Related<super::catalog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Catalog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
