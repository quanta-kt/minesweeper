use actix::{Handler, Message};
use serde::Deserialize;

use super::GameHandler;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "action")]
pub enum PlayerAction {
    #[serde(rename = "flag")]
    Flag { index: usize },
    #[serde(rename = "reveal")]
    Reveal { index: usize },
}

pub enum MoveError {
    NoSuchGame,
    NoSuchPlayer,
    InvalidMove,
}
pub struct PlayerMove {
    pub game_code: u16,
    pub player_code: u16,
    pub action: PlayerAction,
}

impl Message for PlayerMove {
    type Result = Result<(), MoveError>;
}

impl Handler<PlayerMove> for GameHandler {
    type Result = Result<(), MoveError>;

    fn handle(&mut self, msg: PlayerMove, _ctx: &mut Self::Context) -> Self::Result {
        let game = self
            .games
            .get_mut(&msg.game_code)
            .ok_or(MoveError::NoSuchGame)?;

        let player_game = game
            .players
            .get_mut(&msg.player_code)
            .ok_or(MoveError::NoSuchPlayer)?;

        match msg.action {
            PlayerAction::Flag { index } => player_game
                .board
                .toggle_flag(index)
                .map_err(|_| MoveError::InvalidMove)?,

            PlayerAction::Reveal { index } => player_game
                .board
                .reveal(index)
                .map_err(|_| MoveError::InvalidMove)?,
        };

        Ok(())
    }
}
