use actix::Addr;
use actix_web::{error, get, web, HttpRequest, Responder};
use actix_web_actors::ws;
use serde::Deserialize;

use crate::{game_handler, routes::ws::GameWebSocketActor};

#[derive(Deserialize, Debug)]
struct JoinGameQuery {
    code: String,
    player_name: String,
}

#[get("join-game")]
async fn join_game(
    req: HttpRequest,
    query: web::Query<JoinGameQuery>,
    game_handler: web::Data<Addr<game_handler::GameHandler>>,
    stream: web::Payload,
) -> actix_web::Result<impl Responder> {
    let game_code = u16::from_str_radix(&query.code, 16)
        .map_err(|_err| error::ErrorBadRequest("Invalid game code"))?;

    let player_code = game_handler
        .send(game_handler::JoinGame::new(
            game_code,
            query.player_name.to_owned(),
        ))
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?
        .map_err(|err| match err {
            game_handler::JoinGameError::GameFull => error::ErrorBadRequest("Game is full"),
            game_handler::JoinGameError::GameNotFound => {
                error::ErrorNotFound("Unable to find the game")
            }
        })?;

    Ok(ws::start(
        GameWebSocketActor {
            game_handler_addr: game_handler.as_ref().clone(),
            game_code,
            player_code,
        },
        &req,
        stream,
    ))
}
