mod processes;
mod state;
mod to_do;
use std::env;

use processes::process_input;
use serde_json::json;
use state::read_file;
use to_do::{
    enums::TaskStatus,
    to_do_factory,
    traits::{delete::Delete, edit::Edit, get::Get},
    ItemTypes,
};

use crate::state::write_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let title = &args[2];

    let mut state = read_file("./state.json");
    let status: String;

    match state.get(title) {
        Some(result) => status = result.to_string().replace('\"', ""),
        None => status = "Pending".to_string(),
    }

    let item = to_do_factory(title, TaskStatus::from_string(status));
    process_input(item, command.to_string(), &state);
}
