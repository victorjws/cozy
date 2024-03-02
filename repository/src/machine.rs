use db::sea_orm::*;
use model_entity::{machine, prelude::Machine};

pub struct MachineRepo;

impl MachineRepo {
    pub async fn create(db: &DbConn, name: &str) -> Result<machine::Model, DbErr> {
        let new_machine = machine::ActiveModel {
            name: Set(name.to_owned()),
            ..Default::default()
        };
        let inserted_machine = new_machine.insert(db).await?;
        Ok(inserted_machine)
    }

    pub async fn get_all(db: &DbConn) -> Result<Vec<machine::Model>, DbErr> {
        Machine::find().all(db).await
    }
}
