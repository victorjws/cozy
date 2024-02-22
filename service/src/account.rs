use std::collections::HashMap;

use db::sea_orm::*;
use model_entity::{
    account::Model as Account,
    character::Model as CharacterModel,
    dto::{
        Business as BusinessDTO, BusinessIn, Character as CharacterDTO, Food as FoodDTO,
        Machine as MachineDTO, Mission as MissionDTO,
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

    pub async fn get(db: &DbConn, account_id: i32) -> Result<Option<Account>, DbErr> {
        let account_object = AccountRepo::get_by_id(db, account_id).await?;
        Ok(account_object)
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

    pub async fn update_data(db: &DbConn, account_id: i32, data: &BusinessIn) -> Result<(), DbErr> {
        AccountRepo::get_by_id(db, account_id).await.unwrap();
        BusinessRepo::update(db, account_id, data).await?;
        let characters = CharacterRepo::get_all(db).await?;
        let foods = FoodRepo::get_all(db).await?;
        let machines = MachineRepo::get_all(db).await?;

        let characters_map: HashMap<String, i32> = characters
            .into_iter()
            .map(|model| (model.name, model.id))
            .collect();
        let foods_map: HashMap<String, i32> = foods
            .into_iter()
            .map(|model| (model.name, model.id))
            .collect();
        let machines_map: HashMap<String, i32> = machines
            .into_iter()
            .map(|model| (model.name, model.id))
            .collect();

        if let Some(current_characters) = &data.current_characters {
            let mut remove_character_ids: Vec<i32> = Vec::new();
            for current_character in current_characters {
                let character_id = *characters_map.get(&current_character.name).unwrap();
                if current_character.is_unlocked {
                    CharacterRepo::update_characters(
                        db,
                        account_id,
                        character_id,
                        current_character.current_level,
                    )
                    .await?;
                } else {
                    remove_character_ids.push(character_id);
                }
            }
            if remove_character_ids.len() > 0 {
                CharacterRepo::delete_characters(db, account_id, remove_character_ids).await?;
            }
        }

        if let Some(current_foods) = &data.current_foods {
            let mut remove_food_ids: Vec<i32> = Vec::new();
            for current_food in current_foods {
                let food_id = *foods_map.get(&current_food.name).unwrap();
                if current_food.is_unlocked {
                    FoodRepo::update_foods(db, account_id, food_id, current_food.current_level)
                        .await?;
                } else {
                    remove_food_ids.push(food_id);
                }
            }
            if remove_food_ids.len() > 0 {
                FoodRepo::delete_foods(db, account_id, remove_food_ids).await?;
            }
        }

        if let Some(current_machines) = &data.current_machines {
            let mut remove_machine_ids: Vec<i32> = Vec::new();
            for current_machine in current_machines {
                let machine_id = *machines_map.get(&current_machine.name).unwrap();
                if current_machine.is_unlocked {
                    MachineRepo::update_machines(
                        db,
                        account_id,
                        machine_id,
                        current_machine.current_level,
                    )
                    .await?;
                } else {
                    remove_machine_ids.push(machine_id);
                }
            }
            if remove_machine_ids.len() > 0 {
                MachineRepo::delete_machines(db, account_id, remove_machine_ids).await?;
            }
        }

        if let Some(current_missions) = &data.current_missions {
            let mut remove_mission_ids: Vec<i32> = Vec::new();
            for current_mission in current_missions {
                if current_mission.is_unlocked {
                    MissionRepo::update_missions(
                        db,
                        account_id,
                        current_mission.id,
                        current_mission.is_cleared,
                    )
                    .await?;
                } else {
                    remove_mission_ids.push(current_mission.id);
                }
            }
            if remove_mission_ids.len() > 0 {
                MissionRepo::delete_missions(db, account_id, remove_mission_ids).await?;
            }
        }
        Ok(())
    }
}
