use db::sea_orm::*;
use model_entity::{food, prelude::Food};

pub struct FoodRepo;

impl FoodRepo {
    pub async fn get_all(db: &DbConn) -> Result<Vec<food::Model>, DbErr> {
        Food::find().all(db).await
    }
}
