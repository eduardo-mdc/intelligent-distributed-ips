mod worker;

use crate::config::TrafficConfig;
use crate::stats::Stats;
use anyhow::Result;
use log::info;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

pub struct TrafficGenerator {
    config: Arc<TrafficConfig>,
    stats: Stats,
}

impl TrafficGenerator {
    pub fn new(config: TrafficConfig) -> Result<Self> {
        Ok(Self {
            config: Arc::new(config),
            stats: Stats::new(),
        })
    }

    pub async fn run(self) -> Result<()> {
        info!("Starting traffic generation...");
        info!("Target bandwidth: {:.2} Mbps", self.config.target_bandwidth_bps as f64 / 1_000_000.0);
        info!("Connections: {}", self.config.connections);
        info!("Request size: {} bytes", self.config.request_size);

        // Spawn stats printer
        let stats_clone = self.stats.clone();
        tokio::spawn(async move {
            stats_clone.print_stats_loop().await;
        });

        // Calculate delay between requests to achieve target bandwidth
        let bytes_per_sec = self.config.bytes_per_second();
        let delay_between_requests = if bytes_per_sec > 0 && self.config.request_size > 0 {
            let requests_per_sec = bytes_per_sec as f64 / self.config.request_size as f64;
            let delay_per_conn = 1.0 / (requests_per_sec / self.config.connections as f64);
            Duration::from_secs_f64(delay_per_conn.max(0.001)) // Minimum 1ms
        } else {
            Duration::from_millis(100)
        };

        info!("Delay between requests: {:?}", delay_between_requests);

        // Spawn workers
        let mut handles = Vec::new();
        for worker_id in 0..self.config.connections {
            let config = Arc::clone(&self.config);
            let stats = self.stats.clone();

            let handle = tokio::spawn(async move {
                worker::Worker::new(worker_id, config, stats, delay_between_requests)
                    .run()
                    .await
            });

            handles.push(handle);
        }

        // Wait for duration or total size
        if self.config.run_duration.as_secs() > 0 {
            info!("Running for {:?}", self.config.run_duration);
            timeout(self.config.run_duration, async {
                for handle in handles {
                    let _ = handle.await;
                }
            })
            .await
            .ok();
        } else {
            info!("Running indefinitely (Ctrl+C to stop)");
            for handle in handles {
                let _ = handle.await;
            }
        }

        // Print final stats
        println!("\n=== Final Statistics ===");
        println!("Total requests: {}", self.stats.get_requests());
        println!("Total bytes: {:.2} MB", self.stats.get_bytes() as f64 / 1_000_000.0);
        println!("Total errors: {}", self.stats.get_errors());
        println!("Average bandwidth: {:.2} Mbps", self.stats.bandwidth_mbps());
        println!("Average requests/sec: {:.2}", self.stats.requests_per_sec());
        println!("Duration: {:.2}s", self.stats.elapsed().as_secs_f64());

        Ok(())
    }
}
