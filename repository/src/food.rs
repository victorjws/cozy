use db::sea_orm::*;
use model_entity::{food, prelude::Food};

pub struct FoodRepo;

impl FoodRepo {
    pub async fn create(db: &DbConn, name: &str) -> Result<food::Model, DbErr> {
        let new_food = food::ActiveModel {
            name: Set(name.to_owned()),
            ..Default::default()
        };
        let inserted_food = new_food.insert(db).await?;
        Ok(inserted_food)
    }

    pub async fn get_all(db: &DbConn) -> Result<Vec<food::Model>, DbErr> {
        Food::find().all(db).await
    }
}
