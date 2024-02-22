use db::sea_orm::*;
use model_entity::{character, prelude::Character};

pub struct CharacterRepo;

impl CharacterRepo {
    pub async fn get_all(db: &DbConn) -> Result<Vec<character::Model>, DbErr> {
        Character::find().all(db).await
    }
}
