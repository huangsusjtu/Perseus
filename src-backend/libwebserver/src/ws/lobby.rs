use std::time::Instant;

use actix::{Actor, ActorContext, StreamHandler};
use actix_http::ws::Message;
use actix_web_actors::ws;

pub struct SimulatorSubscriber {
    pub(crate) simulator_name: String,

    pub(crate) heartbeat: Instant,
}

impl SimulatorSubscriber {
    pub fn new(simulator_name: String) -> Self {
        SimulatorSubscriber {
            simulator_name,
            heartbeat: Instant::now(),
        }
    }
}

impl Actor for SimulatorSubscriber {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("SimulatorSubscriber started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("SimulatorSubscriber stopped");
    }
}

// 发消息
// impl Handler<()> for SimulatorSubscriber {
//
// }

// 收消息不做处理
impl StreamHandler<Result<ws::Message, ws::ProtocolError>>
    for SimulatorSubscriber
{
    fn handle(
        &mut self, item: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match item {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };
        self.heartbeat = Instant::now();
        match msg {
            Message::Ping(msg) => ctx.pong(&msg),
            Message::Text(text) => ctx.text(text),
            Message::Binary(bin) => ctx.binary(bin),
            Message::Continuation(_) => {}
            Message::Pong(_) => {}
            Message::Close(r) => {
                ctx.close(r);
                ctx.stop();
            }
            Message::Nop => {}
        }
    }
}
