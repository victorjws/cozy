use db::sea_orm::*;
use model_entity::{business, dto::Business as BusinessDTO};

pub struct BusinessRepo;

impl BusinessRepo {
    pub async fn update(
        db: &DbConn,
        account_id: i32,
        business_data: BusinessDTO,
    ) -> Result<(), DbErr> {
        let am = business::ActiveModel {
            account_id: Set(account_id),
            current_stage_number: Set(business_data.current_stage_number),
            current_stage_money: Set(business_data.current_stage_money),
            current_dia: Set(business_data.current_dia),
            enabled_tables: Set(business_data.enabled_tables),
            chef_speed_multiplier: Set(business_data.chef_speed_multiplier),
            server_speed_multiplier: Set(business_data.server_speed_multiplier),
            accumulated_customer: Set(business_data.accumulated_customer),
            accumulated_sales: Set(business_data.accumulated_sales),
        };
        am.save(db).await?;
        Ok(())
    }
}
