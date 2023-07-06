mod models;
// use log::{info, warn};
use filters::chess_api;
use models::Db;
use warp::Filter;

#[tokio::main]
async fn main() {
    const RUST_LOG: &str = "RUST_LOG";
    if std::env::var_os(RUST_LOG).is_none() {
        std::env::set_var(RUST_LOG, "chess=info");
    }
    pretty_env_logger::init();

    let db: Db = models::create_game();
    println!("{}", db.lock().unwrap());
    let route = chess_api(db);

    //? printing to board for debugging
    //db.print_with_marked(&Position::new_from_index(0, 0));

    //? serve
    warp::serve(route.with(warp::log("chess")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

mod filters {

    use crate::models::{Db, Position};
    use warp::{hyper::StatusCode, Filter};

    pub fn chess_api(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        print(db.clone())
            .or(move_path(db.clone()))
            .or(show_moves(db))
    }

    //? including with database
    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn show_moves(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::get().and(warp::path!("show" / Position).and(with_db(db.clone())).map(
            |pos: Position, db: Db| {
                db.lock().unwrap().print_with_marked(&pos);
                let positions = db.lock().unwrap().show_moves_of_tile(&pos);
                warp::reply::with_status(warp::reply::json(&positions), StatusCode::ACCEPTED)
            },
        ))
    }

    fn move_path(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::get().and(
            warp::path!("move" / Position / Position)
                .and(with_db(db.clone()))
                .map(|start: Position, end: Position, db: Db| {
                    println!("start:{:?} - end:{:?} ", start, end);

                    if let Err(error_message) = db.lock().unwrap().move_piece(&start, &end) {
                        return error_message;
                    }

                    println!("{}", db.lock().unwrap());
                    format!("{:?} - {:?}\n{}", start, end, db.lock().unwrap())
                }),
        )
    }

    fn print(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::get().and(
            warp::path("print")
                .and(with_db(db.clone()))
                .map(|board: Db| {
                    println!("{}", board.lock().unwrap());
                    board.lock().unwrap().to_string()
                }),
        )
    }
}

mod handlers {}
