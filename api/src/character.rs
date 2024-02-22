use crate::auth::User;
use db::sea_orm_rocket::Connection;
use db::Db;
use model_entity::dto::NameIn;
use rocket::response::status;
use rocket::serde::json::Json;
use service::CharacterService;

#[post("/characters", data = "<data>")]
async fn create_character(
    conn: Connection<'_, Db>,
    data: Json<NameIn<'_>>,
    _user: User,
) -> status::Created<()> {
    let db = conn.into_inner();
    let character = CharacterService::create(db, data.name).await.unwrap();
    let url = format!("/characters/{}", character.id);
    status::Created::new(url)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_character]
}
