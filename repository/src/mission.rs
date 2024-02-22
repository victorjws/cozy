use db::sea_orm::*;
use model_entity::{accounts_missions, mission, prelude::Mission};

pub struct MissionRepo;

impl MissionRepo {
    pub async fn create(db: &DbConn, id: i32) -> Result<mission::Model, DbErr> {
        let new_mission = mission::ActiveModel {
            id: Set(id),
            ..Default::default()
        };
        let inserted_mission = new_mission.insert(db).await?;
        Ok(inserted_mission)
    }

    pub async fn get_all(db: &DbConn) -> Result<Vec<mission::Model>, DbErr> {
        Mission::find().all(db).await
    }

    pub async fn update_missions(
        db: &DbConn,
        account_id: i32,
        mission_id: i32,
        is_cleared: Option<bool>,
    ) -> Result<(), DbErr> {
        let mut am = accounts_missions::ActiveModel {
            account_id: Set(account_id),
            mission_id: Set(mission_id),
            ..Default::default()
        };
        if let Some(is_cleared) = is_cleared {
            am.is_cleared = Set(is_cleared);
        }
        accounts_missions::Entity::insert(am)
            .on_conflict(
                sea_query::OnConflict::columns([
                    accounts_missions::Column::AccountId,
                    accounts_missions::Column::MissionId,
                ])
                .update_column(accounts_missions::Column::IsCleared)
                .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn delete_missions(
        db: &DbConn,
        account_id: i32,
        mission_ids: Vec<i32>,
    ) -> Result<(), DbErr> {
        accounts_missions::Entity::delete_many()
            .filter(
                accounts_missions::Column::AccountId
                    .eq(account_id)
                    .and(accounts_missions::Column::MissionId.is_in(mission_ids)),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
