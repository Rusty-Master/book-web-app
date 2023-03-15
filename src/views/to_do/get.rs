use actix_web::{web, Responder};

use crate::{
    json_serialization::to_do_items::ToDoItems,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn get() -> impl Responder {
    ToDoItems::get_state()
}
