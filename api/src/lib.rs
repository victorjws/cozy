#[macro_use]
extern crate rocket;

pub mod account;
pub mod character;
pub mod food;
pub mod machine;
pub mod mission;

use db::sea_orm_rocket::Database;
use db::Db;

#[tokio::main]
pub async fn start() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Db::init())
        .mount("/", account::routes())
        .mount("/", character::routes())
        .mount("/", food::routes())
        .mount("/", machine::routes())
        .mount("/", mission::routes())
        .launch()
        .await
        .map(|_| ())
}
