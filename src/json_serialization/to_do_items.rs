use crate::{
    state::read_file,
    to_do::{
        enums::TaskStatus,
        structs::{base::Base, pending::Pending},
        to_do_factory, ItemTypes,
    },
};
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_items = Vec::new();
        let mut done_items = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(value) => pending_items.push(value.base),
                ItemTypes::Done(value) => done_items.push(value.base),
            }
        }

        let pending_count = pending_items.len() as i8;
        let done_count = done_items.len() as i8;

        ToDoItems {
            pending_items,
            done_items,
            pending_item_count: pending_count,
            done_item_count: done_count,
        }
    }

    pub fn get_state() -> ToDoItems {
        let state = read_file("./state.json");

        let mut items = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
            let item = to_do_factory(&key, status);

            items.push(item);
        }

        ToDoItems::new(items)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
