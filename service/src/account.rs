use db::sea_orm::*;
use model_entity::{dto::Business as BusinessDTO, prelude::AccountsFoods};
use repository::{AccountRepo, CharacterRepo, FoodRepo, MachineRepo, MissionRepo};

// #[derive(Serialize)]
// #[serde(crate = "rocket::serde")]

pub struct AccountService;

impl AccountService {
    pub async fn create_account(db: &DbConn) -> Result<(), DbErr> {
        AccountRepo::create(db).await?;
        Ok(())
    }

    pub async fn get_data(db: &DbConn, account_id: i32) -> Result<BusinessDTO, DbErr> {
        let account = AccountRepo::get(db, account_id).await?;
        let characters = CharacterRepo::get_all(db).await?;
        let foods = FoodRepo::get_all(db).await?;
        let machines = MachineRepo::get_all(db).await?;
        let missions = MissionRepo::get_all(db).await?;

        Ok(account)
    }

    pub async fn update_data(db: &DbConn, account_id: i32, data: BusinessDTO) -> Result<(), DbErr> {
        Ok(())
    }
}
