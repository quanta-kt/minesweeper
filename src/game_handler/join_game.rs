use std::time::SystemTime;

use actix::{Handler, Message};
use rand::Rng;

use super::{GameHandler, WsPlayerGame};

pub struct JoinGame {
    game_code: u16,
    player_name: String,
}

impl JoinGame {
    pub fn new(game_code: u16, player_name: String) -> JoinGame {
        JoinGame {
            game_code,
            player_name,
        }
    }
}

pub enum JoinGameError {
    GameNotFound,
    GameFull,
}

impl Message for JoinGame {
    type Result = Result<u16, JoinGameError>;
}

impl Handler<JoinGame> for GameHandler {
    type Result = Result<u16, JoinGameError>;

    fn handle(&mut self, join_game: JoinGame, _ctx: &mut Self::Context) -> Self::Result {
        let game = self
            .games
            .get_mut(&join_game.game_code)
            .ok_or(JoinGameError::GameNotFound)?;

        if game.players.len() >= game.config.player_limit {
            return Err(JoinGameError::GameFull);
        }

        let player_code = (0..0xff)
            .map(|_| rand::thread_rng().gen_range(0u16..0xffffu16))
            .filter(|code| !game.players.contains_key(&code))
            .next()
            .expect("generate a random code not already present");

        game.players.insert(
            player_code,
            WsPlayerGame {
                board: game.board.clone(),
                name: join_game.player_name,
                start_time: SystemTime::now(),
                finished_time: None,
            },
        );

        Ok(player_code)
    }
}
