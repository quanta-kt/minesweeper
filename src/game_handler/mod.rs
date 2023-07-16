mod create_game;
mod get_game_state;
mod join_game;
mod player_move;

pub use create_game::CreateGame;

pub use get_game_state::GameStateUpdate;
pub use get_game_state::GetGameState;
pub use get_game_state::GetGameStateError;

pub use join_game::JoinGame;
pub use join_game::JoinGameError;

pub use player_move::MoveError;
pub use player_move::PlayerAction;
pub use player_move::PlayerMove;

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::time::SystemTime;

use actix::{Actor, Context};
use serde::Deserialize;

use crate::board;
use crate::board::Board;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct GameConfig {
    pub player_limit: usize,
    pub board_size: usize,
}

#[derive(Debug, Clone)]
pub struct WsGame {
    config: GameConfig,

    // The common unsolved board
    board: Board,

    players: HashMap<u16, WsPlayerGame>,
}

impl WsGame {
    pub fn new(config: GameConfig) -> WsGame {
        WsGame {
            board: Board::generate(config.board_size.clone()),
            players: HashMap::new(),
            config,
        }
    }
}

#[derive(Debug, Clone)]
struct WsPlayerGame {
    board: board::Board,
    name: String,
    start_time: SystemTime,
    finished_time: Option<SystemTime>,
}

pub struct GameHandler {
    games: HashMap<u16, WsGame>,
}

impl Default for GameHandler {
    fn default() -> Self {
        Self {
            games: HashMap::new(),
        }
    }
}

impl Actor for GameHandler {
    type Context = Context<Self>;
}
