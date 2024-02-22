use db::sea_orm;
use db::sea_orm::*;
use model_entity::dto::{
    Business as BusinessDTO, Character as CharacterDTO, Food as FoodDTO, Machine as MachineDTO,
    Mission as MissionDTO,
};
use model_entity::prelude::{
    AccountsCharacters, AccountsFoods, AccountsMachines, AccountsMissions, Business,
};
use model_entity::{
    account, accounts_characters, accounts_foods, accounts_machines, accounts_missions, business,
    character, food, machine, mission,
};

#[derive(Debug, FromQueryResult)]
struct DataJoinResult {
    name: String,
    current_level: i32,
}

impl From<DataJoinResult> for CharacterDTO {
    fn from(join_result: DataJoinResult) -> Self {
        Self {
            name: join_result.name,
            current_level: join_result.current_level,
            is_unlocked: true,
        }
    }
}

impl From<DataJoinResult> for MachineDTO {
    fn from(join_result: DataJoinResult) -> Self {
        Self {
            name: join_result.name,
            current_level: join_result.current_level,
            is_unlocked: true,
        }
    }
}

impl From<DataJoinResult> for FoodDTO {
    fn from(join_result: DataJoinResult) -> Self {
        Self {
            name: join_result.name,
            current_level: join_result.current_level,
            is_unlocked: true,
        }
    }
}

#[derive(Debug, FromQueryResult)]
struct MissionJoinResult {
    id: i32,
    is_cleared: bool,
}

impl From<MissionJoinResult> for MissionDTO {
    fn from(join_result: MissionJoinResult) -> Self {
        Self {
            mission_index: join_result.id,
            is_unlocked: true,
            is_cleared: join_result.is_cleared,
        }
    }
}

pub struct AccountRepo;

impl AccountRepo {
    pub async fn create(db: &DbConn) -> Result<account::Model, DbErr> {
        let new_account = account::ActiveModel {
            ..Default::default()
        };
        let inserted_account = new_account.insert(db).await?;
        Ok(inserted_account)
    }

    pub async fn get(db: &DbConn, account_id: i32) -> Result<BusinessDTO, DbErr> {
        let business_data = Business::find()
            .filter(business::Column::AccountId.eq(account_id)) // Adjust based on your schema
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Business data not found for account".into()))?;

        let characters: Vec<CharacterDTO> = AccountsCharacters::find()
            .filter(accounts_characters::Column::AccountId.eq(account_id))
            .join(
                JoinType::InnerJoin,
                accounts_characters::Relation::Character.def(),
            )
            .select_only()
            .column_as(character::Column::Name, "name")
            .column_as(accounts_characters::Column::CurrentLevel, "current_level")
            .into_model::<DataJoinResult>()
            .all(db)
            .await?
            .into_iter()
            .map(CharacterDTO::from)
            .collect();

        let foods: Vec<FoodDTO> = AccountsFoods::find()
            .filter(accounts_foods::Column::AccountId.eq(account_id))
            .join(JoinType::InnerJoin, accounts_foods::Relation::Food.def())
            .select_only()
            .column_as(food::Column::Name, "name")
            .column_as(accounts_foods::Column::CurrentLevel, "current_level")
            .into_model::<DataJoinResult>()
            .all(db)
            .await?
            .into_iter()
            .map(FoodDTO::from)
            .collect();

        let machines: Vec<MachineDTO> = AccountsMachines::find()
            .filter(accounts_machines::Column::AccountId.eq(account_id))
            .join(
                JoinType::InnerJoin,
                accounts_machines::Relation::Machine.def(),
            )
            .select_only()
            .column_as(machine::Column::Name, "name")
            .column_as(accounts_machines::Column::CurrentLevel, "current_level")
            .into_model::<DataJoinResult>()
            .all(db)
            .await?
            .into_iter()
            .map(MachineDTO::from)
            .collect();

        let missions: Vec<MissionDTO> = AccountsMissions::find()
            .filter(accounts_missions::Column::AccountId.eq(account_id))
            .join(
                JoinType::InnerJoin,
                accounts_missions::Relation::Mission.def(),
            )
            .select_only()
            .column_as(mission::Column::Id, "id")
            .column_as(accounts_missions::Column::IsCleared, "is_cleared")
            .into_model::<MissionJoinResult>()
            .all(db)
            .await?
            .into_iter()
            .map(MissionDTO::from)
            .collect();

        let data = BusinessDTO {
            current_stage_number: business_data.current_stage_number,
            current_stage_money: business_data.current_stage_money,
            current_dia: business_data.current_dia,
            enabled_tables: business_data.enabled_tables,
            chef_speed_multiplier: business_data.chef_speed_multiplier,
            server_speed_multiplier: business_data.server_speed_multiplier,
            current_characters: characters,
            current_foods: foods,
            current_machines: machines,
            current_missions: missions,
        };

        Ok(data)
    }

    pub async fn update_characters(
        db: &DbConn,
        account_id: i32,
        character_id: i32,
        current_level: i32,
    ) -> Result<(), DbErr> {
        let am = accounts_characters::ActiveModel {
            account_id: Set(account_id),
            character_id: Set(character_id),
            current_level: Set(current_level),
        };
        am.save(db).await?;
        Ok(())
    }

    pub async fn update_foods(
        db: &DbConn,
        account_id: i32,
        food_id: i32,
        current_level: i32,
    ) -> Result<(), DbErr> {
        let am = accounts_foods::ActiveModel {
            account_id: Set(account_id),
            food_id: Set(food_id),
            current_level: Set(current_level),
        };
        am.save(db).await?;
        Ok(())
    }

    pub async fn update_machines(
        db: &DbConn,
        account_id: i32,
        machine_id: i32,
        current_level: i32,
    ) -> Result<(), DbErr> {
        let am = accounts_machines::ActiveModel {
            account_id: Set(account_id),
            machine_id: Set(machine_id),
            current_level: Set(current_level),
        };
        am.save(db).await?;
        Ok(())
    }

    pub async fn update_missions(
        db: &DbConn,
        account_id: i32,
        mission_id: i32,
        is_cleared: bool,
    ) -> Result<(), DbErr> {
        let am = accounts_missions::ActiveModel {
            account_id: Set(account_id),
            mission_id: Set(mission_id),
            is_cleared: Set(is_cleared),
        };
        am.save(db).await?;
        Ok(())
    }
}
