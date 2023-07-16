use actix::{Handler, Message};
use rand::Rng;
use serde::Deserialize;

use super::{GameConfig, GameHandler, WsGame};

#[derive(Deserialize)]
pub struct CreateGame {
    pub board_size: usize,
    pub player_limit: usize,
}

impl Message for CreateGame {
    type Result = Result<u16, ()>;
}

impl Handler<CreateGame> for GameHandler {
    type Result = Result<u16, ()>;

    fn handle(&mut self, msg: CreateGame, _ctx: &mut Self::Context) -> Self::Result {
        let code = (0..0xff)
            .map(|_| rand::thread_rng().gen_range(0u16..0xffffu16))
            .filter(|code| !self.games.contains_key(&code))
            .next()
            .expect("generate a random code not already present");

        let new_game = WsGame::new(GameConfig {
            board_size: msg.board_size,
            player_limit: msg.player_limit,
        });

        self.games.insert(code, new_game);

        Ok(code)
    }
}
