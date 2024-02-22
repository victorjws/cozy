use db::sea_orm::*;
use model_entity::{machine, prelude::Machine};

pub struct MachineRepo;

impl MachineRepo {
    pub async fn get_all(db: &DbConn) -> Result<Vec<machine::Model>, DbErr> {
        Machine::find().all(db).await
    }
}
