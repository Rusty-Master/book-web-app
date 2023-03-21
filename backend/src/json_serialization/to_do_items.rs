use crate::database::DBCONNECTION;
use crate::diesel;
use crate::models::item::item::Item;
use crate::{
    database::establish_connection,
    schema::to_do,
    to_do::{enums::TaskStatus, structs::base::Base, to_do_factory, ItemTypes},
};
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
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

    pub fn get_state(user_id: i32) -> ToDoItems {
        let connection = DBCONNECTION.db_connection.get().unwrap();

        let items = to_do::table
            .filter(to_do::columns::user_id.eq(user_id))
            .order(to_do::columns::id.asc())
            .load::<Item>(&connection)
            .unwrap();

        let mut buffer = Vec::with_capacity(items.len());

        for item in items {
            let status = TaskStatus::from_string(item.status);
            let item = to_do_factory(&item.title, status);
            buffer.push(item);
        }

        ToDoItems::new(buffer)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
