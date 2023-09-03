use super::db::DB;
use super::models::Position;
use actix_web::{
    error::ResponseError,
    web::{self, Data, ServiceConfig},
    Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{error::Error, fmt::Display, str::FromStr};

pub fn routes(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::scope("/").route("", web::get().to(health_check)));

    config.service(web::scope("/board").route("", web::get().to(get_board)));
    config.service(web::scope("/move").route("/{from}/{to}", web::get().to(move_piece)));
    config.service(web::scope("/reset").route("", web::get().to(reset_board)));
}

//? health check route

async fn health_check() -> impl Responder {
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

#[derive(Debug, Serialize)]
struct CustomError(String);

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponse::build(self.status_code()).json(self.to_string())
    }
}

async fn move_piece(
    data: web::Data<DB>,
    moved: web::Path<MoveInfo>,
) -> Result<impl Responder, CustomError> {
    let db = data.into_inner();
    let mut board = db.get_board().await;
    let from = Position::from_str(&moved.from).expect("cannot transform into position");
    let to = Position::from_str(&moved.to).expect("cannot transform into position");
    board.move_piece(&from, &to).map_err(|e| CustomError(e))?;
    db.move_piece(from, to)
        .await
        .map_err(|e| CustomError(e.to_string()))?;
    Ok(web::Json("success"))
}

async fn reset_board(data: web::Data<DB>) -> Result<impl Responder, Box<dyn Error>> {
    data.into_inner().reset().await?;
    Ok(web::Json("success"))
}
