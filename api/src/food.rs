use db::sea_orm_rocket::Connection;
use db::Db;
use model_entity::dto::Base;
use rocket::response::status;
use rocket::serde::json::Json;
use service::FoodService;

#[post("/foods", data = "<data>")]
async fn create_food(conn: Connection<'_, Db>, data: Json<Base<'_>>) -> status::Created<()> {
    let db = conn.into_inner();
    let food = FoodService::create(db, data.name).await.unwrap();
    let url = format!("/foods/{}", food.id);
    status::Created::new(url)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_food]
}
