use db::sea_orm::*;
use model_entity::{mission, prelude::Mission};

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
}
