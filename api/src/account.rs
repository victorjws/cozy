// use db::sea_orm::{DbErr, TransactionTrait};
use db::sea_orm_rocket::Connection;
use db::Db;
use model_entity::dto::Business as BusinessDTO;
use rocket::serde::json::Json;

use service::account::AccountService;

#[post("/accounts")]
async fn create_account(conn: Connection<'_, Db>) -> () {
    let db = conn.into_inner();
    AccountService::create_account(db).await.unwrap();
    // status::Created("".to_owned())
}

#[get("/accounts/<id>")]
async fn get_account(conn: Connection<'_, Db>, id: i32) -> Json<BusinessDTO> {
    let db = conn.into_inner();
    let account = AccountService::get_data(db, id).await.unwrap();
    Json(account)
}

#[patch("/accounts/<id>", data = "<data>")]
async fn update_account(conn: Connection<'_, Db>, id: i32, data: Json<BusinessDTO>) -> () {
    let db = conn.into_inner();
    let account = AccountService::get_data(db, id).await.unwrap();
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_account, get_account]
}
