//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "accounts_missions")]
#[serde(crate = "rocket::serde")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub mission_id: i32,
    pub is_cleared: bool,
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
        belongs_to = "super::mission::Entity",
        from = "Column::MissionId",
        to = "super::mission::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Mission,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::mission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Mission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
