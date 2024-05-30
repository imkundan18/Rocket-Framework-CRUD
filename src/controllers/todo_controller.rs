use core::sync;

use bson::to_bson;
/*
use std::intrinsics::needs_drop;
*/
use rocket::serde::json::Json;
use rocket::State;
use crate::models::todo::Todo;
use rocket::http::Status;
use crate::db::mongodb::MongoDB;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{results::{InsertOneResult, UpdateResult, DeleteResult}};
use futures::stream::StreamExt;



#[post("/todo", format = "json", data = "<todo>")]
pub async fn create_todo(db: &State<MongoDB>,todo: Json<Todo>) -> Result<Json<InsertOneResult>,Status> {
    let new_todo = todo.into_inner();
    let d=db.col.insert_one(&new_todo, None).await;
     match d {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// #[get("/todo")]
// pub async fn get_todos(db:&State<MongoDB> ) -> Json<Vec<Todo>> {
//     let mut cursor = db.col.find(None, None).await.unwrap();
//     let mut todos = Vec::new();
//     while let Some(result) = cursor.try_next().await.unwrap() {
//         todos.push(result);
//     }
//     Json(todos)
// }
#[get("/todo/all")]
pub async fn get_all(db:&State<MongoDB>)-> Result<Json<Vec<Todo>>,Status>{

    let mut cursor= match db.col.find(None, None).await
     {
        Ok(cursor) => cursor,
        Err(_) => return Err(Status::InternalServerError),
     };

    let mut todos=Vec::new();
    while let Some(result) =  cursor.next().await{
        match result{
            Ok(todo)=>todos.push(todo),
            Err(_)=>return Err(Status::BadRequest),
        }
        
    }
    Ok(Json(todos))
}



// #[get("/todo/<path>")]
// pub async fn get_todos(db:&State<MongoDB>, path:String ) -> Result<Json<Todo>,Status> {
//     let id=path;
//     if id.is_empty(){
//         return Err(Status::BadRequest);
//     }
//     let obj_id = match ObjectId::parse_str(id) {
//         Ok(oid) => oid,
//         Err(_) => return Err(Status::BadRequest),
//     };
//     let filter=doc!{"_id":obj_id};
//     let detail=db.col.find_one(filter, None).await;
//     match detail {
//         Ok(Some(todo)) => Ok(Json(todo)),
//         Ok(None) => Err(Status::NotFound),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }



#[get("/todo/<path>")]
pub async fn get_todos(db:&State<MongoDB>, path:String ) -> Result<Json<Todo>,Status> {
    let id=path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    }
    // let obj_id = match ObjectId::parse_str(id) {
    //     Ok(oid) => oid,
    //     Err(_) => return Err(Status::BadRequest),
    // };
    let filter=doc!{"title":id};
    let detail=db.col.find_one(filter, None).await;
    match detail {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/todo/<path>", data="<new_data>")]
pub async fn update_todo(db:&State<MongoDB>, new_data:Json<Todo>, path:String)->Result<Json<Todo>, Status>{
    let id=path;
    if id.is_empty(){
        return Err(Status::BadRequest)
    };
    let data=Todo{
        id:Some(ObjectId::parse_str(&id).unwrap()),
        title:new_data.title.to_owned(),
        description:new_data.description.to_owned(),
        completed:new_data.completed,
    };
    let obj_id=ObjectId::parse_str(&id).unwrap();
    let filter=doc!{"_id":obj_id};
    let bson_data=to_bson(&data).unwrap();
    let u_data=doc!{"$set":bson_data};
    let update=db.col.update_one(filter, u_data, None).await;
    match update{
        Ok(_)=>Ok(Json(data)),
        Err(_)=>Err(Status::BadRequest),
    }
}
#[delete("/todo/<path>")]
pub async fn del_data(db:&State<MongoDB>, path:String) -> Result<Json<&str>,Status>{
    let id=path;
    if id.is_empty(){
        return Err(Status::BadRequest);
    };
    let add=ObjectId::parse_str(&id).unwrap();
    let filter=doc!{"_id":add};
    let del=db.col.delete_one(filter, None).await;
    match del{
        Ok(res)=>{
            if res.deleted_count > 0{
                Ok(Json("Deleted"))
            }else{
                Err(Status::BadRequest)
            }
        }
        Err(e) => {
            eprintln!("Failed to delete document: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}