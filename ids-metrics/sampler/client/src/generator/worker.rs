use crate::config::{Protocol, TrafficConfig};
use crate::stats::Stats;
use log::{debug, warn};
use rand::Rng;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{interval, sleep};

pub struct Worker {
    id: usize,
    config: Arc<TrafficConfig>,
    stats: Stats,
    delay: Duration,
    client: reqwest::Client,
}

impl Worker {
    pub fn new(
        id: usize,
        config: Arc<TrafficConfig>,
        stats: Stats,
        delay: Duration,
    ) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self {
            id,
            config,
            stats,
            delay,
            client,
        }
    }

    pub async fn run(&self) {
        let mut tick = interval(self.delay);
        let mut total_sent = 0u64;

        loop {
            tick.tick().await;

            // Check if we've reached total size limit
            if self.config.total_bytes > 0 && total_sent >= self.config.total_bytes {
                debug!("Worker {} reached total size limit", self.id);
                break;
            }

            // Apply request delay/latency
            if self.config.request_delay.as_millis() > 0 {
                sleep(self.config.request_delay).await;
            }

            // Send request
            match self.send_request().await {
                Ok(bytes) => {
                    self.stats.record_request(bytes);
                    total_sent += bytes;
                }
                Err(e) => {
                    warn!("Worker {} request failed: {}", self.id, e);
                    self.stats.record_error();
                }
            }
        }
    }

    async fn send_request(&self) -> anyhow::Result<u64> {
        let url = self.build_url();

        // Generate random payload
        let payload = self.generate_payload();
        let payload_size = payload.len() as u64;

        // Send POST request with payload
        let response = self.client
            .post(&url)
            .body(payload)
            .send()
            .await?;

        // Read response to ensure full request/response cycle
        let _body = response.bytes().await?;

        Ok(payload_size)
    }

    fn build_url(&self) -> String {
        let base = &self.config.target_url;

        // Use httpbin.org endpoints or similar
        match self.config.protocol {
            Protocol::Http => {
                if base.contains("httpbin") {
                    format!("{}/post", base)
                } else {
                    base.clone()
                }
            }
            Protocol::Https => {
                if base.contains("httpbin") {
                    format!("{}/post", base.replace("http://", "https://"))
                } else {
                    base.replace("http://", "https://")
                }
            }
        }
    }

    fn generate_payload(&self) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut payload = Vec::with_capacity(self.config.request_size);

        // Fill with random bytes
        for _ in 0..self.config.request_size {
            payload.push(rng.gen());
        }

        payload
    }
}
