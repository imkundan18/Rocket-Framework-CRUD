use rocket::Route;
use crate::controllers::todo_controller::{create_todo, get_todos};

pub fn get_routes() -> Vec<Route> {
    routes![create_todo, get_todos]
}
