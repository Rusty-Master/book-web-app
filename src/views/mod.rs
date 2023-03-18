use actix_web::web::ServiceConfig;

use self::{auth::auth_views_factory, to_do::to_do_view_factory};

mod app;
mod auth;
mod to_do;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_view_factory(app);
    app::app_views_factory(app);
}
