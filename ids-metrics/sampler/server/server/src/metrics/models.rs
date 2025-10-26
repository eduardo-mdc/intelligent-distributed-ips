use serde::{Deserialize, Serialize};

/// Real-time metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeMetrics {
    // Rates (per second)
    pub bandwidth_bps: u64,
    pub packets_per_sec: u64,
    pub tcp_bandwidth_bps: u64,
    pub tcp_packets_per_sec: u64,
    pub http_bandwidth_bps: u64,
    pub http_packets_per_sec: u64,
    pub https_bandwidth_bps: u64,
    pub https_packets_per_sec: u64,

    // Totals
    pub total_packets: u64,
    pub total_bytes: u64,
}

/// Summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryStats {
    pub uptime_seconds: u64,
    pub avg_bandwidth_bps: u64,
    pub peak_bandwidth_bps: u64,
    pub total_packets: u64,
    pub total_bytes: u64,
    pub tcp_percentage: f64,
    pub http_percentage: f64,
    pub https_percentage: f64,
}

/// Internal metrics snapshot for calculations
#[derive(Debug, Clone, Copy, Default)]
pub struct MetricsSnapshot {
    pub timestamp: u64,
    pub total_packets: u64,
    pub total_bytes: u64,
    pub tcp_packets: u64,
    pub tcp_bytes: u64,
    pub http_packets: u64,
    pub http_bytes: u64,
    pub tls_packets: u64,
    pub tls_bytes: u64,
}
