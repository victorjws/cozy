use db::sea_orm::{DbConn, DbErr};
use model_entity::machine::Model as Machine;
use repository::MachineRepo;

pub struct MachineService;

impl MachineService {
    pub async fn create(db: &DbConn, name: &str) -> Result<Machine, DbErr> {
        let new_machine = MachineRepo::create(db, name).await?;
        Ok(new_machine)
    }
}
