use actix_web::{web::Json, HttpResponse};

use crate::{
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    processes::process_input,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn edit(to_do_item: Json<ToDoItem>, token: JwToken) -> HttpResponse {
    println!("Here is the message in token: {}", token.message);
    let state = read_file("./state.json");
    let status;

    match state.get(&to_do_item.title) {
        Some(result) => status = TaskStatus::from_string(result.as_str().unwrap().to_string()),
        None => {
            return HttpResponse::NotFound().json(format!("{} do not exist.", to_do_item.title))
        }
    }

    if status.to_string() == to_do_item.status {
        return HttpResponse::Ok().json(ToDoItems::get_state());
    }
    let item = to_do_factory(&to_do_item.title, status);

    process_input(item, "edit".to_string(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
