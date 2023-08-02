mod db;
mod models;
// use log::{info, warn};
use db::DB;
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

    //? create connection to DB
    let db_sql = DB::db_start().await;
    let result = sqlx::query!("select * from piece_colors;")
        .fetch_all(&db_sql.connection)
        .await
        .unwrap();
    println!("{:?}", result);
    let db: Db = models::create_game();
    println!("{}", db.lock().unwrap());
    let route = chess_api(db);

    //? printing to board for debugging
    //db.print_with_marked(&Position::new_from_index(0, 0));

    //? serve
    warp::serve(route.with(warp::log("chess")))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

mod filters {

    use crate::models::{Db, Position};
    use cookie::{time::Duration, Cookie};
    use warp::{
        hyper::StatusCode,
        reply::{with_header, with_status},
        Filter,
    };

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
        warp::get().and(warp::path!("show" / Position).and(with_db(db)).map(
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
                .and(with_db(db))
                .and(warp::header::optional("cookie"))
                .map(
                    |start: Position, end: Position, db: Db, user: Option<Cookie>| {
                        println!("start:{:?} - end:{:?} ", start, end);
                        if let Some(cook) = user {
                            println!("{}", cook);
                        }
                        if let Err(error_message) = db.lock().unwrap().move_piece(&start, &end) {
                            return with_status(
                                with_header(error_message, "", ""),
                                StatusCode::BAD_REQUEST,
                            );
                        }
                        if db.lock().unwrap().is_check() {
                            println!("CHECKKKKKKKKKKKKKKKKKKKKKKK")
                        }

                        //todo game could use cookies to store player_color on client
                        //? if the game is designed as a console application, we can't use cookies
                        let cooki = Cookie::build("user", "white")
                            .path("/")
                            .max_age(Duration::days(20))
                            .finish();
                        println!("{}", db.lock().unwrap());
                        with_status(
                            with_header(
                                format!("{:?} - {:?}\n{}", start, end, db.lock().unwrap()),
                                "Set-Cookie",
                                cooki.to_string(),
                            ),
                            StatusCode::OK,
                        )
                    },
                ),
        )
    }

    fn print(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::get().and(warp::path("print").and(with_db(db)).map(|board: Db| {
            println!("{}", board.lock().unwrap());
            board.lock().unwrap().to_string()
        }))
    }
}

mod handlers {}
