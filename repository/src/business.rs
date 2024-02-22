use db::sea_orm::*;
use model_entity::{business, dto::BusinessIn};

pub struct BusinessRepo;

impl BusinessRepo {
    pub async fn create(db: &DbConn, account_id: i32) -> Result<business::Model, DbErr> {
        let new_business = business::ActiveModel {
            account_id: Set(account_id),
            ..Default::default()
        };
        let inserted_business = new_business.insert(db).await?;
        Ok(inserted_business)
    }

    pub async fn update(
        db: &DbConn,
        account_id: i32,
        business_data: &BusinessIn,
    ) -> Result<(), DbErr> {
        let mut am = business::ActiveModel {
            account_id: Set(account_id),
            ..Default::default()
        };
        let mut columns: Vec<business::Column> = vec![];
        if let Some(current_stage_number) = business_data.current_stage_number {
            am.current_stage_number = Set(current_stage_number);
            columns.push(business::Column::CurrentStageNumber);
        }
        if let Some(current_stage_money) = business_data.current_stage_money {
            am.current_stage_money = Set(current_stage_money);
            columns.push(business::Column::CurrentStageMoney);
        }
        if let Some(current_dia) = business_data.current_dia {
            am.current_dia = Set(current_dia);
            columns.push(business::Column::CurrentDia);
        }
        if let Some(enabled_tables) = business_data.enabled_tables {
            am.enabled_tables = Set(enabled_tables);
            columns.push(business::Column::EnabledTables);
        }
        if let Some(chef_speed_multiplier) = business_data.chef_speed_multiplier {
            am.chef_speed_multiplier = Set(chef_speed_multiplier);
            columns.push(business::Column::ChefSpeedMultiplier);
        }
        if let Some(server_speed_multiplier) = business_data.server_speed_multiplier {
            am.server_speed_multiplier = Set(server_speed_multiplier);
            columns.push(business::Column::ServerSpeedMultiplier);
        }
        if let Some(accumulated_customer) = business_data.accumulated_customer {
            am.accumulated_customer = Set(accumulated_customer);
            columns.push(business::Column::AccumulatedCustomer);
        }
        if let Some(accumulated_sales) = business_data.accumulated_sales {
            am.accumulated_sales = Set(accumulated_sales);
            columns.push(business::Column::AccumulatedSales);
        }

        business::Entity::insert(am)
            .on_conflict(
                sea_query::OnConflict::column(business::Column::AccountId)
                    .update_columns(columns)
                    .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
