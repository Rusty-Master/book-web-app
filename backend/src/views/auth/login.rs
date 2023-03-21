use std::collections::HashMap;

use crate::diesel;
use crate::json_serialization::login_response::LoginResponse;
use crate::models::user::user::User;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::DB;
use crate::json_serialization::login::Login;
use crate::jwt::JwToken;
use crate::schema::users;

pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {
    let password = credentials.password.clone();
    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&db.connection)
        .unwrap();

    if users.is_empty() {
        return HttpResponse::NotFound().finish();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().finish();
    }

    match users[0].verify(password) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            let response = LoginResponse {
                token: raw_token.clone(),
            };
            let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok()
                .append_header(("token", raw_token))
                .json(body)
        }
        false => HttpResponse::Unauthorized().finish(),
    }
}
