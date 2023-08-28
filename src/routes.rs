use super::db::DB;
use actix_web::{
    web::{self, Data, ServiceConfig},
    Responder,
};

pub fn routes(config: &mut actix_web::web::ServiceConfig) {
    config.service(web::scope("/").route("", web::get().to(health_check)));
    config.service(web::scope("/test").route("", web::get().to(check_db_query)));
}

//? health check route

async fn health_check() -> impl Responder {
    "success"
}

async fn check_db_query(data: web::Data<DB>) -> impl Responder {
    data.into_inner().query().await;
    "success"
}
