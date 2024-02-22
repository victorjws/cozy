#[macro_use]
extern crate rocket;

pub mod account;
pub mod auth;
pub mod character;
pub mod food;
pub mod machine;
pub mod mission;

use db::sea_orm_rocket::Database;
use db::Db;
use model_entity::config::AppConfig;

#[tokio::main]
pub async fn start() -> Result<(), rocket::Error> {
    let rocket = rocket::build();
    let figment = rocket.figment();
    let secret_key: String = figment
        .extract_inner("secret_key")
        .expect("secret_key must be set");

    rocket
        .manage(AppConfig { secret_key })
        .attach(Db::init())
        .mount("/", account::routes())
        .mount("/", auth::routes())
        .mount("/", character::routes())
        .mount("/", food::routes())
        .mount("/", machine::routes())
        .mount("/", mission::routes())
        .launch()
        .await
        .map(|_| ())
}
