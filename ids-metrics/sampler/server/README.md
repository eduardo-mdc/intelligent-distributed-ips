# Network Metrics Sampler

High-performance TCP packet monitoring using eBPF/XDP for kernel-level packet capture and analysis.

## Overview

This application captures network packets at the kernel level using eBPF (extended Berkeley Packet Filter) and XDP (eXpress Data Path), providing real-time metrics on bandwidth, packet rates, and protocol distribution with minimal overhead.

## Architecture

The project consists of three main packages:

### 1. **server-ebpf** (eBPF Kernel Programs)
Located in `server-ebpf/src/main.rs`

The eBPF program runs directly in the Linux kernel, attached to a network interface via XDP. It:
- Intercepts packets at the earliest point (before sk_buff allocation)
- Parses Ethernet → IPv4 → TCP headers
- Identifies protocols by port number (80=HTTP, 443=HTTPS)
- Updates per-CPU statistics in eBPF maps
- Returns `XDP_PASS` to allow packets to continue normally

**Key components:**
- `PACKET_STATS`: PerCpuArray map storing packet/byte counters
- `try_server()`: Main packet processing function
- `ptr_at()`: Safe packet boundary checking helper

### 2. **server-common** (Shared Data Structures)
Located in `server-common/src/lib.rs`

Defines data structures shared between kernel and userspace:
- `PacketStats`: Cumulative packet/byte counters by protocol
- `FlowKey`: 5-tuple flow identifier (src/dst IP:port + protocol)
- `FlowStats`: Per-flow statistics
- `TcpMetrics`: TCP-specific metrics (RTT, retransmits)

These structures use `#[repr(C)]` for stable ABI and implement `aya::Pod` for safe memory mapping.

### 3. **server** (Userspace Application)
Located in `server/src/`

The userspace program manages the eBPF lifecycle, collects metrics, and serves a web interface.

#### Module Structure:

**`src/main.rs`** - Entry point
- Parses CLI arguments (interface, port)
- Initializes eBPF manager and metrics aggregator
- Spawns background tasks (metrics collection, web server)

**`src/ebpf/`** - eBPF Management
- `loader.rs`: Loads eBPF object, attaches XDP program to interface
- Handles memlock rlimit, eBPF logger initialization
- Supports fallback from native XDP to SKB_MODE for compatibility

**`src/metrics/`** - Metrics Collection & Aggregation
- `collector.rs`: Reads eBPF maps every 1 second, aggregates per-CPU stats
- `aggregator.rs`: Computes per-second rates, tracks historical data
- `models.rs`: Defines API response structures (`RealtimeMetrics`, `SummaryStats`)

**`src/web/`** - HTTP Server & API
- `routes.rs`: Axum router setup, graceful shutdown handling
- `handlers.rs`: API endpoint implementations
- Serves REST API + static dashboard HTML

**`static/index.html`** - Web Dashboard
- Single-page real-time dashboard
- Auto-refreshes metrics every 2 seconds
- Displays bandwidth, packet rates, protocol percentages

## Requirements

- **Linux kernel 5.x+** with eBPF support
- **Root privileges** (required for XDP attachment)
- **Rust toolchain**: nightly + rust-src
- **bpf-linker**: eBPF bytecode linker

## Installation

```bash
# System dependencies (Ubuntu/Debian)
sudo apt install pkg-config libssl-dev build-essential

# Rust toolchains
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# eBPF tooling
cargo install bpf-linker
```

## Building

The build process uses cargo build scripts to automatically compile eBPF programs:

```bash
# Development build
cargo build --package server

# Release build (optimized)
cargo build --release --package server
```

**Build artifacts:**
- eBPF bytecode: Compiled during build, embedded in binary
- Userspace binary: `target/release/server`

## Running

```bash
# Basic usage (eth0, port 3000)
sudo ./target/release/server

# Custom interface
sudo ./target/release/server --iface wlan0

# Custom port
sudo ./target/release/server --port 8080

# With debug logging
RUST_LOG=debug sudo ./target/release/server --iface eth0
```

**Note:** Root privileges (`sudo`) are required for XDP attachment.

## Usage

### Web Dashboard
Navigate to: `http://localhost:3000`

The dashboard displays:
- **Total Bandwidth** (Mbps) - All network traffic
- **Packets/Second** - Overall packet rate
- **TCP Bandwidth** (Mbps) - TCP traffic only
- **HTTPS Bandwidth** (Mbps) - Port 443 traffic
- **Summary Statistics** - Totals and protocol percentages

### REST API

#### Real-time Metrics
```bash
GET /api/metrics/realtime
```
Returns current per-second rates:
```json
{
  "bandwidth_bps": 125000000,
  "packets_per_sec": 5000,
  "tcp_bandwidth_bps": 120000000,
  "tcp_packets_per_sec": 4800,
  "http_bandwidth_bps": 10000000,
  "http_packets_per_sec": 500,
  "https_bandwidth_bps": 110000000,
  "https_packets_per_sec": 4300,
  "total_packets": 1250000,
  "total_bytes": 1500000000
}
```

#### Summary Statistics
```bash
GET /api/stats/summary
```
Returns aggregated statistics:
```json
{
  "uptime_seconds": 3600,
  "avg_bandwidth_bps": 100000000,
  "peak_bandwidth_bps": 150000000,
  "total_packets": 1250000,
  "total_bytes": 1500000000,
  "tcp_percentage": 95.5,
  "http_percentage": 8.2,
  "https_percentage": 87.3
}
```

#### Health Check
```bash
GET /health
```
Returns: `200 OK`

## Metrics Explained

### Network-Level (from XDP)
- **Total Bandwidth**: All captured traffic (bytes/sec × 8 = bits/sec)
- **Packet Rate**: Packets processed per second
- **TCP Traffic**: Traffic using TCP protocol (IPPROTO_TCP = 6)
- **HTTP Traffic**: TCP packets on port 80
- **HTTPS Traffic**: TCP packets on port 443

### Aggregated Statistics
- **Uptime**: Seconds since server started
- **Average Bandwidth**: Total bytes ÷ uptime
- **Peak Bandwidth**: Highest observed bandwidth
- **Protocol Percentages**: % of total bytes by protocol

## How It Works

```
┌─────────────────────────────────┐
│  Network Interface (eth0)       │
│             ↓                    │
│  XDP Hook (kernel)              │
│             ↓                    │
│  server-ebpf Program            │
│   - Parse packets               │
│   - Update PACKET_STATS map     │
│   - Return XDP_PASS             │
│             ↓                    │
│  PerCpuArray (eBPF map)         │
│             ↓                    │
│  Metrics Collector (userspace) │
│   - Read map every 1s           │
│   - Aggregate CPUs              │
│             ↓                    │
│  Metrics Aggregator             │
│   - Calculate rates             │
│   - Track history               │
│             ↓                    │
│  Axum Web Server                │
│   - REST API                    │
│   - Dashboard UI                │
└─────────────────────────────────┘
```

1. **Packet Capture**: XDP program attached to interface intercepts all packets
2. **Parsing**: eBPF code safely parses packet headers in kernel
3. **Accounting**: Updates per-CPU statistics atomically
4. **Collection**: Userspace reads maps periodically (1 Hz)
5. **Aggregation**: Computes rates from counter deltas
6. **Serving**: REST API and dashboard provide access to metrics

## License

Dual licensed under MIT or Apache-2.0 (same as Aya framework).

eBPF code is dual licensed under GPL-2.0 or MIT.

## Credits

Built with:
- [Aya](https://github.com/aya-rs/aya) - Pure Rust eBPF library
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
