use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Query},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

mod board;

#[get("/")]
async fn get_board_state(test_game: web::Data<Mutex<board::Board>>) -> impl Responder {
    let test_game = test_game.lock().unwrap();
    web::Json(test_game.get_external_state())
}

#[derive(Deserialize)]
struct Pos {
    index: usize,
}

#[post("flag")]
async fn flag(test_game: web::Data<Mutex<board::Board>>, pos: Query<Pos>) -> impl Responder {
    let mut test_game = test_game.lock().unwrap();

    let _ = test_game.toggle_flag(pos.index);
    HttpResponse::Ok()
}

#[post("reveal")]
async fn reveal(test_game: web::Data<Mutex<board::Board>>, pos: Query<Pos>) -> impl Responder {
    let mut test_game = test_game.lock().unwrap();

    let _ = test_game.reveal(pos.index);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let test_game = web::Data::new(Mutex::new(board::Board::generate(8)));

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(cors)
            .service(get_board_state)
            .service(flag)
            .service(reveal)
            .app_data(test_game.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
