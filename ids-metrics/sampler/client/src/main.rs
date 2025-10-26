mod config;
mod generator;
mod stats;

use anyhow::Result;
use clap::Parser;
use log::info;

#[derive(Debug, Parser)]
#[clap(name = "sampler-client")]
#[clap(about = "Network traffic generator for testing metrics sampler", long_about = None)]
struct Cli {
    /// Path to config file (TOML format)
    #[clap(short = 'f', long)]
    config: Option<String>,

    /// Target server URL (overrides config)
    #[clap(short, long)]
    target: Option<String>,

    /// Target bandwidth (e.g., 1MB/s, 10Mbps, 100KB/s) (overrides config)
    #[clap(short, long)]
    bandwidth: Option<String>,

    /// Number of concurrent connections (overrides config)
    #[clap(short, long)]
    connections: Option<usize>,

    /// Total data to send (e.g., 100MB, 1GB, 0 for infinite) (overrides config)
    #[clap(short = 's', long)]
    total_size: Option<String>,

    /// Request latency/delay between requests (e.g., 100ms, 1s) (overrides config)
    #[clap(short, long)]
    latency: Option<String>,

    /// Duration to run (e.g., 30s, 5m, 0 for infinite) (overrides config)
    #[clap(short, long)]
    duration: Option<String>,

    /// Protocol type (http or https) (overrides config)
    #[clap(short, long)]
    protocol: Option<String>,

    /// Request size per request (e.g., 1KB, 10KB, 1MB) (overrides config)
    #[clap(short, long)]
    request_size: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    info!("Starting traffic generator");

    // Load and merge configuration
    let config = config::TrafficConfig::load(cli)?;

    info!("Configuration:");
    info!("  Target: {}", config.target_url);
    info!("  Bandwidth: {:.2} Mbps", config.target_bandwidth_bps as f64 / 1_000_000.0);
    info!("  Connections: {}", config.connections);
    info!("  Request size: {} bytes", config.request_size);
    info!("  Latency: {:?}", config.request_delay);
    info!("  Protocol: {:?}", config.protocol);

    // Create and run traffic generator
    let generator = generator::TrafficGenerator::new(config)?;
    generator.run().await?;

    Ok(())
}
