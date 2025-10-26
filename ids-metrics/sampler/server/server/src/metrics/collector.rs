use super::MetricsAggregator;
use crate::ebpf::EbpfManager;
use aya::maps::PerCpuArray;
use log::{debug, warn};
use server_common::PacketStats;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

/// Start the metrics collection loop
pub async fn start_collection_loop(
    ebpf_manager: Arc<RwLock<EbpfManager>>,
    metrics_aggregator: Arc<RwLock<MetricsAggregator>>,
) {
    let mut tick = interval(Duration::from_secs(1));

    loop {
        tick.tick().await;

        // Collect stats from eBPF
        let stats = match collect_ebpf_stats(&ebpf_manager).await {
            Ok(stats) => stats,
            Err(e) => {
                warn!("Failed to collect eBPF stats: {}", e);
                continue;
            }
        };

        // Update aggregator
        let mut aggregator = metrics_aggregator.write().await;
        aggregator.update(stats);

        debug!(
            "Collected metrics: {} packets, {} bytes",
            stats.total_packets, stats.total_bytes
        );
    }
}

/// Collect and aggregate packet stats from all CPUs
async fn collect_ebpf_stats(
    ebpf_manager: &Arc<RwLock<EbpfManager>>,
) -> anyhow::Result<PacketStats> {
    let manager = ebpf_manager.read().await;
    let ebpf = manager.ebpf();

    // Get the PACKET_STATS map
    let map = ebpf
        .map("PACKET_STATS")
        .ok_or_else(|| anyhow::anyhow!("PACKET_STATS map not found"))?;

    let packet_stats_map: PerCpuArray<_, PacketStats> = PerCpuArray::try_from(map)?;

    // Aggregate stats from all CPUs
    let mut total_stats = PacketStats {
        total_packets: 0,
        total_bytes: 0,
        tcp_packets: 0,
        tcp_bytes: 0,
        tls_packets: 0,
        tls_bytes: 0,
        http_packets: 0,
        http_bytes: 0,
    };

    if let Ok(per_cpu_stats) = packet_stats_map.get(&0, 0) {
        for stats in per_cpu_stats.iter() {
            total_stats.total_packets += stats.total_packets;
            total_stats.total_bytes += stats.total_bytes;
            total_stats.tcp_packets += stats.tcp_packets;
            total_stats.tcp_bytes += stats.tcp_bytes;
            total_stats.tls_packets += stats.tls_packets;
            total_stats.tls_bytes += stats.tls_bytes;
            total_stats.http_packets += stats.http_packets;
            total_stats.http_bytes += stats.http_bytes;
        }
    }

    Ok(total_stats)
}
