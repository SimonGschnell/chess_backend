use super::db::DB;
use super::models::Position;
use actix_web::{
    web::{self, Data, ServiceConfig},
    Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

pub fn routes(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::scope("/").route("", web::get().to(health_check)));
    config.service(web::scope("/test").route("", web::get().to(check_db_query)));
    config.service(web::scope("/board").route("", web::get().to(get_board)));
    config.service(web::scope("/move").route("/{from}/{to}", web::get().to(move_piece)));
}

//? health check route

async fn health_check() -> impl Responder {
    "success"
}

async fn check_db_query(data: web::Data<DB>) -> impl Responder {
    data.into_inner().query().await;
    "success"
}

async fn get_board(data: web::Data<DB>) -> impl Responder {
    let pieces = data.into_inner().print().await;
    web::Json(pieces)
}

#[derive(Deserialize)]
struct MoveInfo {
    from: String,
    to: String,
}

async fn move_piece(data: web::Data<DB>, moved: web::Path<MoveInfo>) -> impl Responder {
    let from = Position::from_str(&moved.from).expect("cannot transform into position");
    let to = Position::from_str(&moved.to).expect("cannot transform into position");
    //todo print from to -> println!("from : {:?}, to: {:?}", from, to);
    //todo data.into_inner().move_piece(from, to).await;
    "to be implemented"
}

async fn smth_else(data: web::Data<DB>) -> impl Responder {
    let board = data.into_inner().get_board().await;
    web::Json(board)
}
