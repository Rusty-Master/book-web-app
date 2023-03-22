mod config;
mod counter;
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
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");

    let site_counter = counter::Counter { count: 0 };
    site_counter.save();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap_fn(|req, srv| {
                let passed: bool = req.path().contains(&format!("/{}/", ALLOWED_VERSION));

                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count += 1;
                site_counter.save();

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
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
