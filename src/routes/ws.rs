use crate::{game::ExternalCell, game_handler};
use actix::{
    fut, Actor, ActorContext, ActorFuture, ActorFutureExt, Addr, ContextFutureSpawner,
    StreamHandler, WrapFuture,
};
use actix_web_actors::ws::{self, CloseReason};
use serde::Serialize;

pub struct GameWebSocketActor {
    pub game_handler_addr: Addr<game_handler::GameHandler>,
    pub game_code: u16,
    pub player_code: u16,
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
                .then(|_res, act, _ctx| act.send_game_state())
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
