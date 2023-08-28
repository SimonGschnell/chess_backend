mod db;
//mod filters;
mod models;
// use log::{info, warn};
use actix_web::{web, App, HttpServer, Responder};
use db::DB;
//use filters::chess_api;
use models::Db;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const RUST_LOG: &str = "RUST_LOG";
    if std::env::var_os(RUST_LOG).is_none() {
        std::env::set_var(RUST_LOG, "chess=info");
    }
    pretty_env_logger::init();

    //? create connection to DB
    let db_sql = DB::db_start().await;

    //let db: Db = models::create_game();
    //println!("{}", db.lock().unwrap());
    // let route = chess_api(db);

    let data = web::Data::new(db_sql);
    HttpServer::new(move || App::new().app_data(data.clone()).configure(routes::routes))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
