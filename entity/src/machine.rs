//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "machine")]
#[serde(crate = "rocket::serde")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::accounts_machines::Entity")]
    AccountsMachines,
}

impl Related<super::accounts_machines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountsMachines.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        super::accounts_machines::Relation::Account.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::accounts_machines::Relation::Machine.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
