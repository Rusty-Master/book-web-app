use actix_web::{web, HttpResponse};

use crate::{
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    processes::process_input,
    state::read_file,
    to_do::{self, enums::TaskStatus, to_do_factory},
};

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let state = read_file("./state.json");
    let status: TaskStatus;
    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());
        }
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title))
        }
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());
    process_input(existing_item, "delete".to_string(), &state);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
