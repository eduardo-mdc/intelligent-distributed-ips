use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::interval;

#[derive(Clone)]
pub struct Stats {
    pub requests_sent: Arc<AtomicU64>,
    pub bytes_sent: Arc<AtomicU64>,
    pub errors: Arc<AtomicU64>,
    pub start_time: Instant,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            requests_sent: Arc::new(AtomicU64::new(0)),
            bytes_sent: Arc::new(AtomicU64::new(0)),
            errors: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }

    pub fn record_request(&self, bytes: u64) {
        self.requests_sent.fetch_add(1, Ordering::Relaxed);
        self.bytes_sent.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_requests(&self) -> u64 {
        self.requests_sent.load(Ordering::Relaxed)
    }

    pub fn get_bytes(&self) -> u64 {
        self.bytes_sent.load(Ordering::Relaxed)
    }

    pub fn get_errors(&self) -> u64 {
        self.errors.load(Ordering::Relaxed)
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn requests_per_sec(&self) -> f64 {
        let elapsed = self.elapsed().as_secs_f64();
        if elapsed == 0.0 {
            return 0.0;
        }
        self.get_requests() as f64 / elapsed
    }

    pub fn bandwidth_bps(&self) -> u64 {
        let elapsed = self.elapsed().as_secs();
        if elapsed == 0 {
            return 0;
        }
        (self.get_bytes() * 8) / elapsed
    }

    pub fn bandwidth_mbps(&self) -> f64 {
        self.bandwidth_bps() as f64 / 1_000_000.0
    }

    pub async fn print_stats_loop(self) {
        let mut tick = interval(Duration::from_secs(1));
        let mut last_requests = 0u64;
        let mut last_bytes = 0u64;

        loop {
            tick.tick().await;

            let current_requests = self.get_requests();
            let current_bytes = self.get_bytes();
            let errors = self.get_errors();

            let requests_delta = current_requests - last_requests;
            let bytes_delta = current_bytes - last_bytes;
            let bandwidth_mbps = (bytes_delta * 8) as f64 / 1_000_000.0;

            println!(
                "[{}s] Requests: {} ({}/s) | Bandwidth: {:.2} Mbps ({:.2} MB/s) | Total: {:.2} MB | Errors: {}",
                self.elapsed().as_secs(),
                current_requests,
                requests_delta,
                bandwidth_mbps,
                bytes_delta as f64 / 1_000_000.0,
                current_bytes as f64 / 1_000_000.0,
                errors
            );

            last_requests = current_requests;
            last_bytes = current_bytes;
        }
    }
}
