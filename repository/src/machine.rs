use db::sea_orm::*;
use model_entity::{accounts_machines, machine, prelude::Machine};

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

    pub async fn update_machines(
        db: &DbConn,
        account_id: i32,
        machine_id: i32,
        current_level: Option<i32>,
    ) -> Result<(), DbErr> {
        let mut am = accounts_machines::ActiveModel {
            account_id: Set(account_id),
            machine_id: Set(machine_id),
            ..Default::default()
        };
        if let Some(current_level) = current_level {
            am.current_level = Set(current_level);
        }
        accounts_machines::Entity::insert(am)
            .on_conflict(
                sea_query::OnConflict::columns([
                    accounts_machines::Column::AccountId,
                    accounts_machines::Column::MachineId,
                ])
                .update_column(accounts_machines::Column::CurrentLevel)
                .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn delete_machines(
        db: &DbConn,
        account_id: i32,
        machine_ids: Vec<i32>,
    ) -> Result<(), DbErr> {
        accounts_machines::Entity::delete_many()
            .filter(
                accounts_machines::Column::AccountId
                    .eq(account_id)
                    .and(accounts_machines::Column::MachineId.is_in(machine_ids)),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
