//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "accounts_machines")]
#[serde(crate = "rocket::serde")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub machine_id: i32,
    pub current_level: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::AccountId",
        to = "super::account::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Account,
    #[sea_orm(
        belongs_to = "super::machine::Entity",
        from = "Column::MachineId",
        to = "super::machine::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Machine,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::machine::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Machine.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
