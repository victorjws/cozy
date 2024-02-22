use db::sea_orm::*;
use model_entity::{accounts_characters, character, prelude::Character};

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

    pub async fn update_characters(
        db: &DbConn,
        account_id: i32,
        character_id: i32,
        current_level: Option<i32>,
    ) -> Result<(), DbErr> {
        let mut am = accounts_characters::ActiveModel {
            account_id: Set(account_id),
            character_id: Set(character_id),
            ..Default::default()
        };
        if let Some(current_level) = current_level {
            am.current_level = Set(current_level);
        }
        accounts_characters::Entity::insert(am)
            .on_conflict(
                sea_query::OnConflict::columns([
                    accounts_characters::Column::AccountId,
                    accounts_characters::Column::CharacterId,
                ])
                .update_column(accounts_characters::Column::CurrentLevel)
                .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn delete_characters(
        db: &DbConn,
        account_id: i32,
        character_ids: Vec<i32>,
    ) -> Result<(), DbErr> {
        accounts_characters::Entity::delete_many()
            .filter(
                accounts_characters::Column::AccountId
                    .eq(account_id)
                    .and(accounts_characters::Column::CharacterId.is_in(character_ids)),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
