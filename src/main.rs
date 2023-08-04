mod db;
mod filters;
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

    let db: Db = models::create_game();
    println!("{}", db.lock().unwrap());
    let route = chess_api(db, &db_sql);

    //? printing to board for debugging
    //db.print_with_marked(&Position::new_from_index(0, 0));

    //? serve
    warp::serve(route.with(warp::log("chess")))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
