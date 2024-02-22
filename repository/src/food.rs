use db::sea_orm::*;
use model_entity::{accounts_foods, food, prelude::Food};

pub struct FoodRepo;

impl FoodRepo {
    pub async fn create(db: &DbConn, name: &str) -> Result<food::Model, DbErr> {
        let new_food = food::ActiveModel {
            name: Set(name.to_owned()),
            ..Default::default()
        };
        let inserted_food = new_food.insert(db).await?;
        Ok(inserted_food)
    }

    pub async fn get_all(db: &DbConn) -> Result<Vec<food::Model>, DbErr> {
        Food::find().all(db).await
    }

    pub async fn update_foods(
        db: &DbConn,
        account_id: i32,
        food_id: i32,
        current_level: Option<i32>,
    ) -> Result<(), DbErr> {
        let mut am = accounts_foods::ActiveModel {
            account_id: Set(account_id),
            food_id: Set(food_id),
            ..Default::default()
        };
        if let Some(current_level) = current_level {
            am.current_level = Set(current_level);
        }
        accounts_foods::Entity::insert(am)
            .on_conflict(
                sea_query::OnConflict::columns([
                    accounts_foods::Column::AccountId,
                    accounts_foods::Column::FoodId,
                ])
                .update_column(accounts_foods::Column::CurrentLevel)
                .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn delete_foods(
        db: &DbConn,
        account_id: i32,
        food_ids: Vec<i32>,
    ) -> Result<(), DbErr> {
        accounts_foods::Entity::delete_many()
            .filter(
                accounts_foods::Column::AccountId
                    .eq(account_id)
                    .and(accounts_foods::Column::FoodId.is_in(food_ids)),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
