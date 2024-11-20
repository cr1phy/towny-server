use std::str::FromStr;

use actix_web::{get, post, web, HttpResponse};
use entity::prelude::*;
use sea_orm::EntityTrait;
use serde_json::json;
use uuid::Uuid;

use crate::{
    forms::{CreateAccount, DeleteAccount},
    mutation::Mutation,
    query::Query,
    AppState,
};

#[get("/")]
async fn status(state: web::Data<AppState>) -> HttpResponse {
    let db = &state.conn;
    let count_accs = Account::find().all(db).await.unwrap().len();

    let body = json! {{"version": env!("CARGO_PKG_VERSION"), "all_users": count_accs}}.take();

    HttpResponse::Ok().json(body)
}

#[get("/server")]
async fn servers_list() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/server/{id}")]
async fn server_info() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/account/{id}")]
async fn get_account(state: web::Data<AppState>, input: web::Path<String>) -> HttpResponse {
    let db = &state.conn;
    let id = input.into_inner();

    if let Err(err) = Uuid::from_str(&id) {
        log::error!("{}", err);
        return HttpResponse::BadRequest().json("Invalid ID.");
    }
    let id = Uuid::from_str(&id).unwrap();

    return match Query::find_account_by_id(db, id).await {
        Ok(opt) => match opt {
            Some(acc) => HttpResponse::Ok().json(acc),
            None => HttpResponse::NotFound().json("Account with this ID isn't found."),
        },
        Err(_) => HttpResponse::BadGateway().json("Something went wrong."),
    };
}

#[post("/account")]
async fn create_account(
    state: web::Data<AppState>,
    input: web::Json<CreateAccount>,
) -> HttpResponse {
    let db = &state.conn;
    let input = input.into_inner();

    return match Mutation::create_account(db, input).await {
        Ok(acc) => HttpResponse::Ok().json(acc.id),
        Err(err) => {
            log::error!("{}", err);
            HttpResponse::BadRequest().json("Something went wrong.")
        }
    };
}

#[post("/account/delete")]
async fn delete_account(
    state: web::Data<AppState>,
    input: web::Json<DeleteAccount>,
) -> HttpResponse {
    let db = &state.conn;
    let input = input.into_inner();

    return match Mutation::delete_account(db, input).await {
        Ok(_) => HttpResponse::Ok().json("This account is deleted."),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    };
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(status)
        .service(servers_list)
        .service(get_account)
        .service(create_account)
        .service(delete_account);
}
