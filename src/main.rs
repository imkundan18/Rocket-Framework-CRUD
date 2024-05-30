#[macro_use] extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
mod controllers;
mod db;
mod routes;
mod models;
//use rocket::{config::Config, routes};
use db::mongodb::MongoDB;
use controllers::todo_controller::{create_todo, get_todos,update_todo,del_data};

//use crate::controllers::todo_controller::{create_todo, get_todos};

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let db =MongoDB::init_db().await;

//     let config = Config::figment()
//         .merge(("port", 8000))
//         .merge(("address", "0.0.0.0"));

//     let _ = rocket::custom(config)
//         .manage(db)
//         .mount("/", routes::todo_routes::get_routes())
//         .launch()
//         .await?;

//     Ok(())
// }

#[launch]
async fn rocket() ->_ {
    let dbase =MongoDB::init_db().await;

    rocket::build().manage(dbase)
    .mount("/", routes![create_todo])
    .mount("/", routes![get_todos])    
    .mount("/", routes![update_todo])
    .mount("/", routes![del_data])
}
