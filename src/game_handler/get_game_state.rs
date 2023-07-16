use std::time;

use actix::{Handler, Message};
use serde::Serialize;

use crate::game::ExternalCell;

use super::GameHandler;

pub struct GetGameState {
    pub game_code: u16,
    pub player_code: u16,
}

#[derive(Debug)]
pub enum GetGameStateError {
    GameNotFound,
    PlayerNotFound,
}

#[derive(Serialize, Debug)]
pub struct GameStateUpdate {
    board_state: Vec<ExternalCell>,
    board_size: usize,
    start_time: Option<u64>,
    finished_time: Option<u64>,
}

impl Message for GetGameState {
    type Result = Result<GameStateUpdate, GetGameStateError>;
}

impl Handler<GetGameState> for GameHandler {
    type Result = Result<GameStateUpdate, GetGameStateError>;

    fn handle(&mut self, msg: GetGameState, _ctx: &mut Self::Context) -> Self::Result {
        let player_game = self
            .games
            .get(&msg.game_code)
            .ok_or(GetGameStateError::GameNotFound)?
            .players
            .get(&msg.player_code)
            .ok_or(GetGameStateError::PlayerNotFound)?;

        Ok(GameStateUpdate {
            board_state: player_game.board.get_external_state(),
            board_size: player_game.board.size(),
            start_time: Some(
                player_game
                    .start_time
                    .duration_since(time::UNIX_EPOCH)
                    .expect("time went backwards")
                    .as_secs(),
            ),
            finished_time: None,
        })
    }
}
