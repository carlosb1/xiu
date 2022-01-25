use {
    //https://rustcc.cn/article?id=6dcbf032-0483-4980-8bfe-c64a7dfb33c7
    anyhow::Result,
    //env_logger::{Builder, Target},
    hls::server as hls_server,
    httpflv::server as httpflv_server,

    rtmp::{
        channels::channels::ChannelsManager,
        relay::{pull_client::PullClient, push_client::PushClient},
        rtmp::RtmpServer,
    },
    std::env,
    tokio,
    tokio::signal,
    xiu::config::{config, config::Config},
};

//use application::logger::logger;
use hls::rtmp_event_processor::RtmpEventProcessor;
use service::*;

mod service;
#[tokio::main]

async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let cfg_path = &args[1];
    let config = config::load(cfg_path);

    match config {
        Ok(val) => {
            /*set log level*/

            // flexi_logger::Logger::try_with_env_or_str("info")?.start()?;
            // if let Some(log_config_value) = &val.log {
            //     flexi_logger::Logger::try_with_env_or_str(log_config_value.level.clone())?
            //         .start()?;
            // }
            if let Some(log_config_value) = &val.log {
                env::set_var("RUST_LOG", log_config_value.level.clone());
            } else {
                env::set_var("RUST_LOG", "info");
            }

            // let mut builder = Builder::from_default_env();
            // builder
            //     .target(Target::Pipe(Box::new(logger::FileTarget::new(
            //         logger::Rotate::Minute,
            //         String::from("./logs"),
            //     ))))
            //     .init();

            env_logger::init();

            /*run the service*/
            let mut serivce = Service::new(val);
            serivce.run().await?;
        }
        _ => (),
    }

    // log::info!("log info...");
    // log::warn!("log warn...");
    // log::error!("log err...");
    // log::trace!("log trace...");
    // log::debug!("log debug...");

    signal::ctrl_c().await?;
    Ok(())
}
