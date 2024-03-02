use db::sea_orm::{DbConn, DbErr};
use model_entity::mission::Model as Mission;
use repository::MissionRepo;

pub struct MissionService;

impl MissionService {
    pub async fn create(db: &DbConn, id: i32) -> Result<Mission, DbErr> {
        let new_mission = MissionRepo::create(db, id).await?;
        Ok(new_mission)
    }
}
