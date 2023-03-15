mod json_serialization;
mod processes;
mod state;
mod to_do;
mod views;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(views::views_factory))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
