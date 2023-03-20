mod config;
mod database;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod to_do;
mod views;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap_fn(|req, srv| {
                println!("{}-{}", req.method(), req.uri());
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
