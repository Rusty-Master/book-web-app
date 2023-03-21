use actix_web::web::ServiceConfig;

use self::{
    app::app_views_factory, auth::auth_views_factory, to_do::to_do_view_factory,
    users::user_views_factory,
};

mod app;
mod auth;
mod to_do;
mod users;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_view_factory(app);
    app_views_factory(app);
    user_views_factory(app);
}
