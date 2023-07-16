use std::collections::HashMap;

use actix::{Actor, Handler, Message};

use super::{CreateGame, GameConfig, GameHandler, WsGame};

struct GetSnapshot;

impl Message for GetSnapshot {
    type Result = Result<HashMap<u16, WsGame>, ()>;
}

impl Handler<GetSnapshot> for GameHandler {
    type Result = Result<HashMap<u16, WsGame>, ()>;
    fn handle(&mut self, _msg: GetSnapshot, _ctx: &mut Self::Context) -> Self::Result {
        Ok(self.games.clone())
    }
}

#[actix_rt::test]
async fn can_create_game() {
    let game_handler = GameHandler::default();
    let game_handler_addr = game_handler.start();

    let result = game_handler_addr
        .send(CreateGame {
            board_size: 8,
            player_limit: 3,
        })
        .await;

    assert!(result.is_ok());

    let result = result.unwrap();
    assert!(result.is_ok());

    let id = result.unwrap();

    let games = game_handler_addr.send(GetSnapshot).await.unwrap().unwrap();

    assert_eq!(games.len(), 1);

    let config = &games.get(&id).unwrap().config;

    assert_eq!(
        *config,
        GameConfig {
            board_size: 8,
            player_limit: 3
        }
    );
}
