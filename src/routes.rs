use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use actix::{Actor, StreamHandler};
use actix_web::{error, get, post, web, HttpRequest, Responder, Result};
use actix_web_actors::ws;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::board::{self, ExternalCell};

struct GameWebSocketActor {
    player_data: Arc<RwLock<WsPlayerGame>>,
}

#[derive(Debug)]
pub struct WsGame {
    config: NewGameConfig,
    players: HashMap<u16, Arc<RwLock<WsPlayerGame>>>,
}

#[derive(Debug)]
struct WsPlayerGame {
    board: board::Board,
    name: String,
    start_time: SystemTime,
    finished_time: Option<SystemTime>,
}

#[derive(Serialize, Debug)]
struct GameStateUpdate {
    board_state: Vec<ExternalCell>,
    board_size: usize,
    start_time: Option<u64>,
    finished_time: Option<u64>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "action")]
enum PlayerAction {
    #[serde(rename = "flag")]
    Flag { index: usize },
    #[serde(rename = "reveal")]
    Reveal { index: usize },
}

impl Actor for GameWebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameWebSocketActor {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(message)) = item {
            let action = serde_json::from_str(&message);

            if action.is_err() {
                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Unsupported,
                    description: Some("Invalid payload to websocket".to_owned()),
                }));

                return;
            }

            let action = action.unwrap();

            let player_game = &mut self.player_data.write();
            let player_game = player_game.as_mut().unwrap();

            match action {
                PlayerAction::Flag { index } => player_game.board.toggle_flag(index).unwrap(),
                PlayerAction::Reveal { index } => player_game.board.reveal(index).unwrap(),
            }

            send_game_state(player_game, ctx);
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        let player_game = self.player_data.read();
        let player_game = player_game.as_ref().unwrap();
        send_game_state(player_game, ctx);
    }
}

fn send_game_state(player_game: &WsPlayerGame, ctx: &mut <GameWebSocketActor as Actor>::Context) {
    let state = GameStateUpdate {
        board_state: player_game.board.get_external_state(),
        board_size: player_game.board.size(),
        start_time: Some(
            player_game
                .start_time
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs(),
        ),
        finished_time: None,
    };

    let json = serde_json::to_string(&state).expect("serializes GameStateUpdate");
    ctx.text(json);
}

#[derive(Deserialize, Debug)]
struct JoinGameQuery {
    code: String,
    name: String,
}

#[get("join-game")]
async fn join_game(
    req: HttpRequest,
    query: web::Query<JoinGameQuery>,
    games: web::Data<RwLock<HashMap<u16, WsGame>>>,
    stream: web::Payload,
) -> Result<impl Responder> {
    let code = u16::from_str_radix(&query.code, 16)
        .map_err(|_| error::ErrorBadRequest("Invalid game code"))?;

    let mut games = games.write().unwrap();

    let game = games
        .get_mut(&code)
        .ok_or(error::ErrorNotFound("Can't find that game"))?;

    if game.players.len() >= game.config.players_limit {
        return Err(error::ErrorBadRequest("Game is already full"));
    }

    let player_code = (0..0xff)
        .map(|_| rand::thread_rng().gen_range(0u16..0xffffu16))
        .filter(|player_code| !game.players.contains_key(player_code))
        .next()
        .expect("generate a player code not already present");

    let player_data = Arc::new(RwLock::new(WsPlayerGame {
        board: board::Board::generate(game.config.board_size),
        name: query.name.to_owned(),
        start_time: SystemTime::now(),
        finished_time: None,
    }));

    game.players.insert(player_code, player_data.clone());

    ws::start(
        GameWebSocketActor {
            player_data: player_data.clone(),
        },
        &req,
        stream,
    )
}

#[derive(Debug, Deserialize)]
struct NewGameConfig {
    board_size: usize,
    players_limit: usize,
}

#[derive(Debug, Serialize)]
struct NewGameResponse {
    code: String,
}
#[post("create-game")]
async fn create_game(
    config: web::Json<NewGameConfig>,
    games: web::Data<RwLock<HashMap<u16, WsGame>>>,
) -> impl Responder {
    let mut games = games.write().unwrap();

    let code = (0..0xff)
        .map(|_| rand::thread_rng().gen_range(0u16..0xffffu16))
        .filter(|code| !games.contains_key(&code))
        .next()
        .expect("generate a random code not already present");

    games.insert(
        code,
        WsGame {
            config: config.0,
            players: HashMap::new(),
        },
    );

    web::Json(NewGameResponse {
        code: format!("{:X}", code),
    })
}

pub fn routes() -> actix_web::Scope {
    web::scope("/api").service(create_game).service(join_game)
}
