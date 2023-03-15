use actix_web::HttpRequest;

use crate::{
    processes::process_input,
    state::read_file,
    to_do::{enums::TaskStatus, to_do_factory},
};

pub async fn create(req: HttpRequest) -> String {
    let state = read_file("./state.json");
    let title = req.match_info().get("title").unwrap().to_string();

    let item = to_do_factory(&title.as_str(), TaskStatus::Pending);
    process_input(item, "create".to_string(), &state);
    format!("{title} created")
}
