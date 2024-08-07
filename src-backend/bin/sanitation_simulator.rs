use std::path::PathBuf;

use clap::{Args, Parser};
use clap_handler::{handler, Handler};
use libutil::log;
use libsimulator::AppState;


#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> anyhow::Result<()> {
    let main = Commands::parse();
    main.run()
}

///  cmd
#[derive(Parser, Handler, Debug, Clone)]
pub enum Commands {
    #[command(name = "serve")]
    #[command(
        about = "start online server",
        long_about = "Run online http server"
    )]
    Serve(ServeCommand),

    #[command(name = "local")]
    #[command(
        about = "start local server",
        long_about = "Run local http server"
    )]
    Local(LocalCommand),
}

/// online server, 会依赖线上数据库，存储
#[derive(Args, Debug, Clone)]
pub struct ServeCommand {
    #[arg(
        short,
        long,
        value_name = "config file path",
        help = "config file path"
    )]
    config: PathBuf,
}

#[handler(ServeCommand)]
pub fn online_serve(_cmd: ServeCommand) -> anyhow::Result<()> {
    // 加载配置

    // 启动服务器
    // todo huangsu
    Ok(())
}

/// local server， 仅仅使用本地存储空间
#[derive(Args, Debug, Clone)]
pub struct LocalCommand {
    #[arg(
        short = 'p',
        long = "port",
        value_name = "service port",
        help = "service port",
        default_value = "2025"
    )]
    port: String,

    #[arg(
        short = 'm',
        long = "map",
        value_name = "local map file path",
        help = "local map file path"
    )]
    map_path: PathBuf,

    #[arg(
        short = 's',
        long = "scenario",
        value_name = "local scenario file path",
        help = "local scenario file path"
    )]
    scenario_path: PathBuf,
}

#[handler(LocalCommand)]
pub fn local_server(cmd: LocalCommand) -> anyhow::Result<()> {
    let log_cleaner = log::init_log()?;

    let eventbus = libutil::eventbus::EventBus::default();
    let apps_state = AppState {
        eventbus: eventbus.clone(),
        map_svc: libsimulator::FileBasedMapServiceImpl::new(cmd.map_path)?,
        scenario_svc: libsimulator::FileSanitationScenarioServiceImpl::new(
            cmd.scenario_path,
        )?,
        sim_svc: libsimulator::SanitationSimulatorServiceImpl::new(eventbus.clone()),
    };

    if let Err(e) =
        libwebserver::start_local_server(cmd.port.parse().unwrap(), apps_state)
    {
        tracing::error!("sever error: {:#?}", e);
        return Err(e);
    }
    log_cleaner();
    Ok(())
}
