mod handlers;
mod config;
mod errors;
mod db;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::add_user;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users").route(web::post().to(add_user)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
