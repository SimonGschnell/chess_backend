mod db;
//mod filters;
mod models;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use db::DB;
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
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .configure(routes::routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
