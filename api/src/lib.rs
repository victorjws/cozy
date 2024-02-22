#[macro_use]
extern crate rocket;

pub mod account;

use db::sea_orm_rocket::Database;
use db::Db;

#[tokio::main]
pub async fn start() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Db::init())
        .mount("/", account::routes())
        .launch()
        .await
        .map(|_| ())
}
