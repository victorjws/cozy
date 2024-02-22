use crate::auth::User;
use db::sea_orm_rocket::Connection;
use db::Db;
use model_entity::dto::IdIn;
use rocket::response::status;
use rocket::serde::json::Json;
use service::MissionService;

#[post("/missions", data = "<data>")]
async fn create_mission(
    conn: Connection<'_, Db>,
    data: Json<IdIn>,
    _user: User,
) -> status::Created<()> {
    let db = conn.into_inner();
    let mission = MissionService::create(db, data.id).await.unwrap();
    let url = format!("/missions/{}", mission.id);
    status::Created::new(url)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_mission]
}
