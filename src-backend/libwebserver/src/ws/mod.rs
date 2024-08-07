mod lobby;

use actix_web::{get, web, Error, HttpRequest, HttpResponse};
pub use lobby::SimulatorSubscriber;

#[get("/{simulator}")]
pub async fn ws_client(
    req: HttpRequest, stream: web::Payload, simulator: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let resp = actix_web_actors::ws::start(
        SimulatorSubscriber::new(simulator.to_string()),
        &req,
        stream,
    );
    tracing::info!("websocket client connect {:?}", simulator.to_string());
    resp
}
