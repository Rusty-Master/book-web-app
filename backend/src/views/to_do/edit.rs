use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;
use crate::schema::to_do;
use actix_web::web::Json;
use actix_web::HttpResponse;
use diesel::prelude::*;

pub async fn edit(to_do_item: Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let connection = establish_connection();
    let results = to_do::table.filter(to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("Done"))
        .execute(&connection);

    HttpResponse::Ok().json(ToDoItems::get_state())
}