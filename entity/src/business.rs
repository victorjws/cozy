//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "business")]
#[serde(crate = "rocket::serde")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub account_id: i32,
    pub current_stage_number: i32,
    pub current_stage_money: i32,
    pub current_dia: i32,
    pub enabled_tables: i32,
    #[sea_orm(column_type = "Float")]
    pub chef_speed_multiplier: f32,
    #[sea_orm(column_type = "Float")]
    pub server_speed_multiplier: f32,
    pub accumulated_customer: i32,
    pub accumulated_sales: i32,
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
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
