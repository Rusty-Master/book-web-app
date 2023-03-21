use actix_web::web::{post, route, scope, ServiceConfig};

mod create;

pub fn user_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/user").route("create", post().to(create::create)));
}
