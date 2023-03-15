use actix_web::web::{get, post, scope, ServiceConfig};

mod create;
mod get;

pub fn to_do_view_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(get::get)),
    );
}
