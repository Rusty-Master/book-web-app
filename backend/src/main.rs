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

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap_fn(|req, srv| {
                let passed: bool = req.path().contains(&format!("/{}/", ALLOWED_VERSION));
                println!("{}-{}", req.method(), req.uri());

                let end_result = if passed {
                    Either::Left(srv.call(req))
                } else {
                    let resp = HttpResponse::NotImplemented()
                        .body(format!("only {} API is supported", ALLOWED_VERSION));
                    Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                };

                async move {
                    let result = end_result.await?;
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
