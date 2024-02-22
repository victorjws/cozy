use crate::auth::User;
use db::sea_orm_rocket::Connection;
use db::Db;
use model_entity::dto::{Business as BusinessDTO, BusinessIn};
use rocket::response::status;
use rocket::serde::json::Json;
use service::AccountService;

#[post("/accounts")]
async fn create_account(conn: Connection<'_, Db>) -> status::Created<()> {
    let db = conn.into_inner();
    let account = AccountService::create(db).await.unwrap();
    let url = format!("/accounts/{}", account.id);
    status::Created::new(url)
}

#[get("/accounts/me")]
async fn get_account(conn: Connection<'_, Db>, user: User) -> Json<BusinessDTO> {
    let db = conn.into_inner();
    let account = AccountService::get_data(db, user.id).await.unwrap();
    Json(account)
}

#[patch("/accounts/me", data = "<data>")]
async fn update_account(
    conn: Connection<'_, Db>,
    user: User,
    data: Json<BusinessIn>,
) -> status::NoContent {
    let db = conn.into_inner();
    AccountService::update_data(db, user.id, &data.into_inner())
        .await
        .unwrap();
    status::NoContent
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_account, get_account, update_account]
}
