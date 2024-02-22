use db::sea_orm::{DbConn, DbErr};
use model_entity::food::Model as Food;
use repository::FoodRepo;

pub struct FoodService;

impl FoodService {
    pub async fn create(db: &DbConn, name: &str) -> Result<Food, DbErr> {
        let new_food = FoodRepo::create(db, name).await?;
        Ok(new_food)
    }
}
