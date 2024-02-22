use db::sea_orm::{DbConn, DbErr};
use model_entity::character::Model as Character;
use repository::CharacterRepo;

pub struct CharacterService;

impl CharacterService {
    pub async fn create(db: &DbConn, name: &str) -> Result<Character, DbErr> {
        let new_character = CharacterRepo::create(db, name).await?;
        Ok(new_character)
    }
}
