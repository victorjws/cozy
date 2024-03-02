use db::sea_orm::*;
use model_entity::{character, prelude::Character};

pub struct CharacterRepo;

impl CharacterRepo {
    pub async fn create(db: &DbConn, name: &str) -> Result<character::Model, DbErr> {
        let new_character = character::ActiveModel {
            name: Set(name.to_owned()),
            ..Default::default()
        };
        let inserted_character = new_character.insert(db).await?;
        Ok(inserted_character)
    }

    pub async fn get_all(db: &DbConn) -> Result<Vec<character::Model>, DbErr> {
        Character::find().all(db).await
    }
}
