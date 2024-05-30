use mongodb::{Client, Collection};
use crate::models::todo::Todo;

// pub async fn init_db() -> Collection<Todo> {
//     let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
//     let database = client.database("todo_db");
//     database.collection::<Todo>("todos")
// }
pub struct MongoDB{
    pub col:Collection<Todo>,
}
impl MongoDB{
    pub async fn init_db()->Self{
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let database = client.database("todo_db");
        let col=database.collection::<Todo>("todos");
        MongoDB{col}
    }
}