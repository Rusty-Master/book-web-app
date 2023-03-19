use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) {
        let item = state.get(title);
        match item {
            Some(item) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", item);
            }
            None => println!("item: {} was not found", title),
        }
    }
}
