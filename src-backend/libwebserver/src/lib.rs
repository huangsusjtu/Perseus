

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use libsimulator::AppState;

mod api;
mod ws;

#[actix_web::main]
pub async fn start_local_server(
    port: u16, app_data: AppState,
) -> anyhow::Result<()> {
    let app_data = actix_web::web::Data::new(app_data);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .expose_any_header()
            .allow_any_method()
            .allow_any_origin()
            .max_age(7200);
        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .app_data(actix_web::web::PayloadConfig::new(256 * 1024 * 1024))
            .configure(api::init_api)
    })
    .workers(2)
    .bind(("0.0.0.0", port))?
    .run()
    .await?;
    Ok(())
}

#[actix_web::main]
pub async fn start_web_server(_ip: &str, _port: u16) -> anyhow::Result<()> {
    // let eventbus = libmodel::eventbus::EventBus::default();
    // let apps_state = actix_web::web::Data::new(AppState {
    //     eventbus: Arc::new(Default::default()),
    //     map_svc: libsimulator::FileBasedMapServiceImpl::new(map_path)?,
    //     scenario_svc:
    // libsimulator::FileSanitationScenarioServiceImpl::new(scenario_path)?,
    //     sim_svc:
    // libsimulator::SimulatorServiceInterface::new(eventbus.clone()), });
    //
    // HttpServer::new(move || {
    //     let cors = Cors::default()
    //         .allow_any_header()
    //         .expose_any_header()
    //         .allow_any_method()
    //         .allow_any_origin()
    //         .max_age(7200);
    //     App::new()
    //         .wrap(actix_web::middleware::Logger::default())
    //         .wrap(cors)
    //         .app_data(apps_state.clone())
    //         .app_data(actix_web::web::PayloadConfig::new(256 * 1024 * 1024))
    //         .configure(api::init_api)
    // })
    // .workers(2)
    // .bind((ip, port))?
    // .run()
    // .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
