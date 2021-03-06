//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "replies"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub message_id: Uuid,
    pub parent_id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub modified_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    MessageId,
    ParentId,
    CreatedAt,
    ModifiedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    MessageId,
    ParentId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (Uuid, Uuid);
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Messages2,
    Messages1,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::MessageId => ColumnType::Uuid.def(),
            Self::ParentId => ColumnType::Uuid.def(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def(),
            Self::ModifiedAt => ColumnType::TimestampWithTimeZone.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Messages2 => Entity::belongs_to(super::messages::Entity)
                .from(Column::MessageId)
                .to(super::messages::Column::Id)
                .into(),
            Self::Messages1 => Entity::belongs_to(super::messages::Entity)
                .from(Column::ParentId)
                .to(super::messages::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
