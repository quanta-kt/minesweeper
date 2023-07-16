use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod game;
mod game_handler;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let game_handler_addr = web::Data::new(game_handler::GameHandler::default().start());

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(routes::routes())
            .app_data(game_handler_addr.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
