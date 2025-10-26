use super::models::{MetricsSnapshot, RealtimeMetrics, SummaryStats};
use chrono::Utc;
use server_common::PacketStats;

/// Aggregates and computes metrics from raw eBPF data
pub struct MetricsAggregator {
    start_time: i64,
    last_snapshot: Option<MetricsSnapshot>,
    current_snapshot: MetricsSnapshot,
    peak_bandwidth_bps: u64,
}

impl MetricsAggregator {
    pub fn new() -> Self {
        Self {
            start_time: Utc::now().timestamp(),
            last_snapshot: None,
            current_snapshot: MetricsSnapshot::default(),
            peak_bandwidth_bps: 0,
        }
    }

    /// Update with new packet stats from eBPF
    pub fn update(&mut self, stats: PacketStats) {
        let timestamp = Utc::now().timestamp() as u64;

        self.last_snapshot = Some(self.current_snapshot);
        self.current_snapshot = MetricsSnapshot {
            timestamp,
            total_packets: stats.total_packets,
            total_bytes: stats.total_bytes,
            tcp_packets: stats.tcp_packets,
            tcp_bytes: stats.tcp_bytes,
            http_packets: stats.http_packets,
            http_bytes: stats.http_bytes,
            tls_packets: stats.tls_packets,
            tls_bytes: stats.tls_bytes,
        };
    }

    /// Get current real-time metrics (rates)
    pub fn get_realtime_metrics(&self) -> RealtimeMetrics {
        let (bytes_per_sec, packets_per_sec, tcp_bytes_per_sec, tcp_packets_per_sec,
             http_bytes_per_sec, http_packets_per_sec, tls_bytes_per_sec, tls_packets_per_sec) =
            self.calculate_rates();

        RealtimeMetrics {
            bandwidth_bps: bytes_per_sec * 8, // Convert to bits
            packets_per_sec,
            tcp_bandwidth_bps: tcp_bytes_per_sec * 8,
            tcp_packets_per_sec,
            http_bandwidth_bps: http_bytes_per_sec * 8,
            http_packets_per_sec,
            https_bandwidth_bps: tls_bytes_per_sec * 8,
            https_packets_per_sec: tls_packets_per_sec,
            total_packets: self.current_snapshot.total_packets,
            total_bytes: self.current_snapshot.total_bytes,
        }
    }

    /// Get summary statistics
    pub fn get_summary(&self) -> SummaryStats {
        let uptime = (Utc::now().timestamp() - self.start_time) as u64;
        let avg_bandwidth = if uptime > 0 {
            (self.current_snapshot.total_bytes * 8) / uptime
        } else {
            0
        };

        let total_bytes = self.current_snapshot.total_bytes;
        let tcp_percentage = if total_bytes > 0 {
            (self.current_snapshot.tcp_bytes as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let http_percentage = if total_bytes > 0 {
            (self.current_snapshot.http_bytes as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let https_percentage = if total_bytes > 0 {
            (self.current_snapshot.tls_bytes as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };

        SummaryStats {
            uptime_seconds: uptime,
            avg_bandwidth_bps: avg_bandwidth,
            peak_bandwidth_bps: self.peak_bandwidth_bps,
            total_packets: self.current_snapshot.total_packets,
            total_bytes,
            tcp_percentage,
            http_percentage,
            https_percentage,
        }
    }

    /// Calculate per-second rates
    fn calculate_rates(&self) -> (u64, u64, u64, u64, u64, u64, u64, u64) {
        let Some(last) = self.last_snapshot else {
            return (0, 0, 0, 0, 0, 0, 0, 0);
        };

        let time_delta = self.current_snapshot.timestamp.saturating_sub(last.timestamp);
        if time_delta == 0 {
            return (0, 0, 0, 0, 0, 0, 0, 0);
        }

        let bytes_diff = self.current_snapshot.total_bytes.saturating_sub(last.total_bytes);
        let packets_diff = self.current_snapshot.total_packets.saturating_sub(last.total_packets);
        let tcp_bytes_diff = self.current_snapshot.tcp_bytes.saturating_sub(last.tcp_bytes);
        let tcp_packets_diff = self.current_snapshot.tcp_packets.saturating_sub(last.tcp_packets);
        let http_bytes_diff = self.current_snapshot.http_bytes.saturating_sub(last.http_bytes);
        let http_packets_diff = self.current_snapshot.http_packets.saturating_sub(last.http_packets);
        let tls_bytes_diff = self.current_snapshot.tls_bytes.saturating_sub(last.tls_bytes);
        let tls_packets_diff = self.current_snapshot.tls_packets.saturating_sub(last.tls_packets);

        (
            bytes_diff / time_delta,
            packets_diff / time_delta,
            tcp_bytes_diff / time_delta,
            tcp_packets_diff / time_delta,
            http_bytes_diff / time_delta,
            http_packets_diff / time_delta,
            tls_bytes_diff / time_delta,
            tls_packets_diff / time_delta,
        )
    }
}
