use db::sea_orm::*;
use model_entity::{mission, prelude::Mission};

pub struct MissionRepo;

impl MissionRepo {
    pub async fn get_all(db: &DbConn) -> Result<Vec<mission::Model>, DbErr> {
        Mission::find().all(db).await
    }
}
