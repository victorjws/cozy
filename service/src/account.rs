use db::sea_orm::*;
use model_entity::{
    account::Model as Account,
    character::Model as CharacterModel,
    dto::{
        Business as BusinessDTO, Character as CharacterDTO, Food as FoodDTO, Machine as MachineDTO,
        Mission as MissionDTO,
    },
    food::Model as FoodModel,
    machine::Model as MachineModel,
    mission::Model as MissionModel,
};
use repository::{AccountRepo, BusinessRepo, CharacterRepo, FoodRepo, MachineRepo, MissionRepo};

pub struct AccountService;

impl AccountService {
    pub async fn create(db: &DbConn) -> Result<Account, DbErr> {
        let new_account = AccountRepo::create(db).await?;
        Ok(new_account)
    }

    async fn merge_characters(
        existing_items: Vec<CharacterDTO>,
        all_items: Vec<CharacterModel>,
    ) -> Vec<CharacterDTO> {
        let mut merged_items: Vec<CharacterDTO> = existing_items;

        for item in all_items.iter() {
            if !merged_items
                .iter()
                .any(|existing_item| existing_item.id == item.id)
            {
                merged_items.push(CharacterDTO::from(item.clone()));
            }
        }
        merged_items.sort_by_key(|item| item.id);
        merged_items
    }

    async fn merge_foods(existing_items: Vec<FoodDTO>, all_items: Vec<FoodModel>) -> Vec<FoodDTO> {
        let mut merged_items: Vec<FoodDTO> = existing_items;

        for item in all_items.iter() {
            if !merged_items
                .iter()
                .any(|existing_item| existing_item.id == item.id)
            {
                merged_items.push(FoodDTO::from(item.clone()));
            }
        }
        merged_items.sort_by_key(|item| item.id);
        merged_items
    }

    async fn merge_machines(
        existing_items: Vec<MachineDTO>,
        all_items: Vec<MachineModel>,
    ) -> Vec<MachineDTO> {
        let mut merged_items: Vec<MachineDTO> = existing_items;

        for item in all_items.iter() {
            if !merged_items
                .iter()
                .any(|existing_item| existing_item.id == item.id)
            {
                merged_items.push(MachineDTO::from(item.clone()));
            }
        }
        merged_items.sort_by_key(|item| item.id);
        merged_items
    }

    async fn merge_missions(
        existing_items: Vec<MissionDTO>,
        all_items: Vec<MissionModel>,
    ) -> Vec<MissionDTO> {
        let mut merged_items: Vec<MissionDTO> = existing_items;

        for item in all_items.iter() {
            if !merged_items
                .iter()
                .any(|existing_item| existing_item.id == item.id)
            {
                merged_items.push(MissionDTO::from(item.clone()));
            }
        }
        merged_items.sort_by_key(|item| item.id);
        merged_items
    }

    pub async fn get_data(db: &DbConn, account_id: i32) -> Result<BusinessDTO, DbErr> {
        let account = AccountRepo::get(db, account_id).await?;

        let all_characters = CharacterRepo::get_all(db).await?;
        let merged_characters =
            AccountService::merge_characters(account.current_characters, all_characters).await;

        let all_foods = FoodRepo::get_all(db).await?;
        let merged_foods = AccountService::merge_foods(account.current_foods, all_foods).await;

        let all_machines = MachineRepo::get_all(db).await?;
        let merged_machines =
            AccountService::merge_machines(account.current_machines, all_machines).await;

        let all_missions = MissionRepo::get_all(db).await?;
        let merged_missions =
            AccountService::merge_missions(account.current_missions, all_missions).await;

        let updated_account = BusinessDTO {
            current_characters: merged_characters,
            current_foods: merged_foods,
            current_machines: merged_machines,
            current_missions: merged_missions,
            ..account
        };

        Ok(updated_account)
    }

    pub async fn update_data(db: &DbConn, account_id: i32, data: BusinessDTO) -> Result<(), DbErr> {
        AccountRepo::get_by_id(db, account_id).await.unwrap();
        BusinessRepo::update(db, account_id, data).await?;
        Ok(())
    }
}
