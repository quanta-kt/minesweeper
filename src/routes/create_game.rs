use actix::Addr;
use actix_web::{error, post, web, Responder};
use serde::Serialize;

use crate::game_handler::{self, CreateGame};

#[derive(Debug, Serialize)]
struct NewGameResponse {
    code: String,
}

#[post("create-game")]
async fn create_game(
    config: web::Json<CreateGame>,
    game_handler: web::Data<Addr<game_handler::GameHandler>>,
) -> actix_web::Result<impl Responder> {
    let config = config.into_inner();

    let code = game_handler
        .send(config)
        .await
        .map_err(|_| error::ErrorInternalServerError("Something went terribly wrong."))?
        .map_err(|_| error::ErrorInternalServerError("Something went terribly wrong."))?;

    Ok(web::Json(NewGameResponse {
        code: format!("{:X}", code),
    }))
}
