use super::db::DB;
use super::models::Position;
use actix_web::{
    error::ResponseError,
    web::{self, Data, ServiceConfig},
    Responder,
};
use serde::Serialize;
use serde_json::json;
use std::{error::Error, fmt::Display};

pub fn routes(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::scope("/").route("", web::get().to(health_check)));

    config.service(
        web::scope("/board")
            .route("", web::get().to(get_board))
            .route("check", web::get().to(is_check))
            .route("reset", web::get().to(reset_board)),
    );
    config.service(
        web::scope("/move")
            //? the order of these routes matter because the second could consume the first
            .route("/show/{position}", web::get().to(show_move))
            .route("/{from}/{to}", web::get().to(move_piece)),
    );
    config.service(web::scope("/reset").route("", web::get().to(reset_board)));
}

//? health check route
async fn health_check() -> impl Responder {
    "success"
}

async fn get_board(data: web::Data<DB>) -> impl Responder {
    let data = data.into_inner();
    let pieces = data.print().await;
    let player_turn = data.get_player_turn().await.unwrap();
    web::Json(json!({"player_turn":player_turn, "board":pieces}))
}

async fn is_check(data: web::Data<DB>) -> impl Responder {
    let board = data.into_inner().get_board().await;
    let mut checkmate: bool = false;
    if board.check_for_checkmate(board.players_turn.clone()) {
        checkmate = true;
    }
    let pos = board.is_check();
    let is_check = match pos.is_some() {
        false => String::from("false"),
        true => String::from("true"),
    };
    web::Json(json!({"is_check": is_check, "is_checkmate": checkmate, "pos": pos}))
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
    moved: web::Path<(Position, Position)>,
) -> Result<impl Responder, CustomError> {
    let db = data.into_inner();
    let mut board = db.get_board().await;
    let (from, to) = moved.into_inner();

    board.move_piece(&from, &to).map_err(|e| CustomError(e))?;

    db.move_piece(from, to)
        .await
        .map_err(|e| CustomError(e.to_string()))?;

    if board.check_for_checkmate(board.players_turn.clone()) {
        return Ok(web::Json("checkmate"));
    }

    Ok(web::Json("success"))
}

async fn show_move(
    data: web::Data<DB>,
    pos: web::Path<Position>,
) -> Result<impl Responder, CustomError> {
    let db = data.into_inner();
    let board = db.get_board().await;
    let pos = pos.into_inner();
    let moves = board.show_moves_of_tile(&pos);
    let moves = moves
        .iter()
        .map(|pos| pos.to_string())
        .collect::<Vec<String>>();

    Ok(web::Json(moves))
}

async fn reset_board(data: web::Data<DB>) -> Result<impl Responder, Box<dyn Error>> {
    data.into_inner().reset().await?;
    Ok(web::Json("success"))
}
