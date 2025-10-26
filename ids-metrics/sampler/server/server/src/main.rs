mod ebpf;
mod metrics;
mod web;

use anyhow::Result;
use clap::Parser;
use log::info;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Parser)]
pub struct Opt {
    /// Network interface to monitor
    #[clap(short, long, default_value = "eth0")]
    pub iface: String,

    /// Web server port
    #[clap(short, long, default_value = "3000")]
    pub port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::parse();

    info!("Starting sampler server on interface: {}", opt.iface);

    // Load and attach eBPF program
    let ebpf_manager = ebpf::EbpfManager::new(&opt.iface).await?;
    info!("eBPF program loaded and attached successfully");

    // Create metrics aggregator
    let metrics_aggregator = metrics::MetricsAggregator::new();

    // Wrap in Arc<RwLock> for shared state
    let ebpf_manager = Arc::new(RwLock::new(ebpf_manager));
    let metrics_aggregator = Arc::new(RwLock::new(metrics_aggregator));

    // Start metrics collection task
    let collector_handle = tokio::spawn(metrics::start_collection_loop(
        ebpf_manager.clone(),
        metrics_aggregator.clone(),
    ));

    // Start web server
    let server_handle = tokio::spawn(web::start_server(
        opt.port,
        metrics_aggregator.clone(),
    ));

    // Wait for either task to complete (or Ctrl+C)
    tokio::select! {
        _ = collector_handle => info!("Metrics collector stopped"),
        _ = server_handle => info!("Web server stopped"),
    }

    info!("Exiting...");
    Ok(())
}
