use serde_json::{Map, Value};

use crate::to_do::{
    structs::{done::Done, pending::Pending},
    traits::{create::Create, delete::Delete, edit::Edit, get::Get},
    ItemTypes,
};

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.base.title, &state),
        "create" => item.create(&item.base.title, &item.base.status.to_string(), &mut state),
        "edit" => item.set_to_done(&item.base.title, &mut state),
        _ => println!("command: {} not supported.", command),
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.base.title, &state),
        "delete" => item.delete(&item.base.title, &mut state),
        "edit" => item.set_to_pending(&item.base.title, &mut state),
        _ => println!("command: {} not supported.", command),
    }
}
