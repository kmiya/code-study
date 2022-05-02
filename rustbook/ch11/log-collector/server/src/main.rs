#[macro_use]
extern crate diesel;

use std::env;

use actix_web::App;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;

mod db;
mod handlers;
mod model;
mod schema;

#[derive(Clone)]
pub struct Server {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Server { pool }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server = Server::new();
    ::actix_web::HttpServer::new(move || {
        use crate::handlers::*;
        App::new()
            .app_data(server.clone())
            .service(handle_post_logs)
            .service(handle_post_csv)
            .service(handle_get_csv)
            .service(handle_get_logs)
    })
    .bind(("localhost", 3000))?
    .run()
    .await
}
