use std::{collections::HashMap, sync::RwLock};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use routes::WsGame;

mod board;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let games = web::Data::new(RwLock::new(HashMap::<u16, WsGame>::new()));

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(routes::routes())
            .app_data(games.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
