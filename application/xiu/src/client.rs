use anyhow::Result;
use hls::rtmp_event_processor::RtmpEventProcessor;
use service::*;
use tokio::signal;
use xiu::config::config::{Config, HlsConfig};

mod service;
#[tokio::main]

async fn main() -> Result<()> {
    let hls_config: Option<HlsConfig> = Some(HlsConfig {
        enabled: true,
        port: 1234,
        ..Default::default()
    });
    let built_config = Config {
        hls: hls_config.clone(),
        ..Default::default()
    };
    let mut service = Service::new(built_config);

    println!("listening in port {:?}", hls_config.unwrap().port);

    service.run().await?;
    signal::ctrl_c().await?;
    Ok(())
}
