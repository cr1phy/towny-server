use std::str::FromStr;

use actix_web::{get, post, web, HttpResponse};
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

    let body = json! {{"status": "Ok!", "version": env!("CARGO_PKG_VERSION")}};

    HttpResponse::Ok().json(body)
}

#[get("/list")]
async fn servers_list() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/account/{id}")]
async fn get_account(state: web::Data<AppState>, input: web::Path<String>) -> HttpResponse {
    let db = &state.conn;
    let id = input.into_inner();

    if let Err(err) = Uuid::from_str(&id) {
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
    println!("{:?}", &input);

    return match Mutation::create_account(db, input).await {
        Ok(acc) => HttpResponse::Ok().json(acc.id),
        Err(err) => {
            println!("{}", err);
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
