use crate::{
    board::ExternalCell,
    game_handler::{self, CreateGame},
};
use actix::{
    fut, Actor, ActorContext, ActorFuture, ActorFutureExt, Addr, ContextFutureSpawner,
    StreamHandler, WrapFuture,
};
use actix_web::{error, get, post, web, HttpRequest, Responder, Result};
use actix_web_actors::ws::{self, CloseReason};
use serde::{Deserialize, Serialize};

struct GameWebSocketActor {
    game_handler_addr: Addr<game_handler::GameHandler>,
    game_code: u16,
    player_code: u16,
}

#[derive(Serialize, Debug)]
struct GameStateUpdate {
    board_state: Vec<ExternalCell>,
    board_size: usize,
    start_time: Option<u64>,
    finished_time: Option<u64>,
}

impl Actor for GameWebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl GameWebSocketActor {
    fn send_game_state(&self) -> impl ActorFuture<Self> {
        self.game_handler_addr
            .send(game_handler::GetGameState {
                player_code: self.player_code,
                game_code: self.game_code,
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                let state = res.unwrap();

                if let Ok(state) = state {
                    let json = serde_json::to_string(&state).expect("serializes GameStateUpdate");
                    ctx.text(json);
                } else {
                    ctx.close(None);
                    ctx.stop();
                }

                fut::ready(())
            })
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameWebSocketActor {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(message)) = item {
            let action = serde_json::from_str(&message);

            if let Err(_) = action {
                ctx.close(Some(CloseReason {
                    code: ws::CloseCode::Invalid,
                    description: Some("Invalid message received over web socket.".to_string()),
                }));

                return;
            }

            let action: game_handler::PlayerAction = action.unwrap();

            self.game_handler_addr
                .send(game_handler::PlayerMove {
                    game_code: self.game_code,
                    player_code: self.player_code,
                    action,
                })
                .into_actor(self)
                .then(|_res, act, ctx| act.send_game_state())
                .then(|_, _, _| fut::ready(()))
                .wait(ctx);
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        self.send_game_state()
            .then(|_, _, _| fut::ready(()))
            .wait(ctx);
    }
}

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
) -> Result<impl Responder> {
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

#[derive(Debug, Serialize)]
struct NewGameResponse {
    code: String,
}

#[post("create-game")]
async fn create_game(
    config: web::Json<CreateGame>,
    game_handler: web::Data<Addr<game_handler::GameHandler>>,
) -> Result<impl Responder> {
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

pub fn routes() -> actix_web::Scope {
    web::scope("/api").service(create_game).service(join_game)
}
