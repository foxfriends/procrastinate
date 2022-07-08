//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "messages"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: Json,
    pub deleted: bool,
    pub created_at: DateTimeWithTimeZone,
    pub modified_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    AuthorId,
    Content,
    Deleted,
    CreatedAt,
    ModifiedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = Uuid;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
    Reactions,
    Attachments,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Uuid.def(),
            Self::AuthorId => ColumnType::Uuid.def(),
            Self::Content => ColumnType::JsonBinary.def(),
            Self::Deleted => ColumnType::Boolean.def(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def(),
            Self::ModifiedAt => ColumnType::TimestampWithTimeZone.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::AuthorId)
                .to(super::users::Column::Id)
                .into(),
            Self::Reactions => Entity::has_many(super::reactions::Entity).into(),
            Self::Attachments => Entity::has_many(super::attachments::Entity).into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::reactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reactions.def()
    }
}

impl Related<super::attachments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attachments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
