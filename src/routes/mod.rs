mod create_game;
mod join_game;
mod ws;

use actix_web::web;

pub fn routes() -> actix_web::Scope {
    web::scope("/api")
        .service(create_game::create_game)
        .service(join_game::join_game)
}
