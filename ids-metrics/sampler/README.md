# Network Metrics Sampler

High-performance network monitoring system using eBPF/XDP for packet capture and analysis.

## Overview

This project consists of two components:

1. **Server** - eBPF-based network monitor that captures and analyzes TCP traffic
2. **Client** - Configurable traffic generator for testing the monitor

## Quick Start

### Prerequisites

```bash
# Ubuntu/Debian
sudo apt install pkg-config libssl-dev build-essential

# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# eBPF requirements (server only)
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo install bpf-linker
```

### Build

```bash
# Server (eBPF monitor)
cd server
cargo build --release

# Client (traffic generator)
cd ../client
cargo build --release
```

### Run

```bash
# Terminal 1: Start server
cd server
sudo ./target/release/server --iface eth0

# Terminal 2: Generate traffic
cd client
./target/release/sampler-client --bandwidth 10Mbps --duration 60s

# Terminal 3: View metrics
# Browser: http://localhost:3000
# Or API: curl http://localhost:3000/api/metrics/realtime
```

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                     Network Traffic                   │
│                           ↓                           │
│  ┌─────────────────────────────────────────────────┐ │
│  │  Server (Monitor)                               │ │
│  │  ┌───────────────────────────────────────────┐ │ │
│  │  │ XDP eBPF Program (kernel)                 │ │ │
│  │  │ - Captures packets at wire speed          │ │ │
│  │  │ - Parses TCP/IP headers                   │ │ │
│  │  │ - Detects HTTP/HTTPS (port-based)         │ │ │
│  │  │ - Updates per-CPU statistics              │ │ │
│  │  └───────────────────────────────────────────┘ │ │
│  │                    ↓                            │ │
│  │  ┌───────────────────────────────────────────┐ │ │
│  │  │ Userspace (Rust)                          │ │ │
│  │  │ - Reads eBPF maps every 1s                │ │ │
│  │  │ - Aggregates multi-CPU stats              │ │ │
│  │  │ - Calculates rates (bandwidth, pps)       │ │ │
│  │  └───────────────────────────────────────────┘ │ │
│  │                    ↓                            │ │
│  │  ┌───────────────────────────────────────────┐ │ │
│  │  │ Axum Web Server                           │ │ │
│  │  │ - REST API endpoints                      │ │ │
│  │  │ - Real-time dashboard UI                  │ │ │
│  │  └───────────────────────────────────────────┘ │ │
│  └─────────────────────────────────────────────────┘ │
│                                                       │
│  ┌─────────────────────────────────────────────────┐ │
│  │  Client (Traffic Generator)                     │ │
│  │  - Multi-threaded HTTP/HTTPS requests          │ │
│  │  - Configurable bandwidth/latency              │ │
│  │  - TOML config + CLI overrides                 │ │
│  └─────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

## Components

### Server (Monitor)

**Purpose**: Capture and analyze network traffic using eBPF/XDP

**Key Features**:
- Kernel-level packet capture (minimal overhead)
- Real-time bandwidth and packet rate metrics
- Protocol detection (HTTP port 80, HTTPS port 443)
- Web dashboard and REST API
- Automatic fallback to SKB mode for compatibility

**Location**: `server/`

**Run**:
```bash
cd server
sudo ./target/release/server --iface eth0 --port 3000
```

**Access**:
- Dashboard: http://localhost:3000
- API: http://localhost:3000/api/metrics/realtime

See [server/README.md](server/README.md) for details.

### Client (Traffic Generator)

**Purpose**: Generate configurable HTTP/HTTPS traffic for testing

**Key Features**:
- TOML configuration file support
- CLI argument overrides
- Configurable bandwidth, latency, connections
- HTTP and HTTPS support
- Real-time statistics

**Location**: `client/`

**Configuration** (`client/client-config.toml`):
```toml
[target]
url = "http://httpbin.org"
protocol = "http"

[traffic]
bandwidth = "10Mbps"
connections = 10
request_size = "1KB"
latency = "0ms"

[limits]
duration = "60s"
total_size = "0"
```

**Run**:
```bash
cd client

# Use config file
./target/release/sampler-client

# Override with CLI
./target/release/sampler-client --bandwidth 50Mbps --duration 30s
```

See [client/README.md](client/README.md) for details.

## How It Works

### Server (Monitoring)

1. **XDP Attachment**: eBPF program attaches to network interface via XDP
2. **Packet Inspection**: For each packet:
   - Parse Ethernet → IPv4 → TCP headers
   - Extract packet size, ports, protocol
   - Identify HTTP (port 80) and HTTPS (port 443)
   - Update per-CPU counters in eBPF map
   - Return `XDP_PASS` (allow packet through)
3. **Metrics Collection**: Userspace reads eBPF maps every second
4. **Aggregation**: Combine stats from all CPUs, calculate rates
5. **Serving**: REST API and dashboard provide real-time access

### Client (Traffic Generation)

1. **Config Loading**: Load TOML config, merge with CLI args
2. **Worker Spawning**: Create N concurrent tokio tasks
3. **Request Loop**: Each worker:
   - Generates random payload of configured size
   - Sends HTTP POST request to target
   - Delays to maintain target bandwidth
   - Tracks requests and bytes sent
4. **Statistics**: Print per-second stats and final summary

## Example Workflow

### Test 1: Basic Monitoring

```bash
# Start server
cd server
sudo ./target/release/server --iface eth0

# Browse the web normally, see your traffic in dashboard
# Open browser: http://localhost:3000
```

### Test 2: Controlled Load Test

```bash
# Terminal 1: Server
cd server
sudo ./target/release/server --iface eth0

# Terminal 2: Generate 10 Mbps for 60 seconds
cd client
./target/release/sampler-client --bandwidth 10Mbps --duration 60s

# Watch the dashboard update in real-time
```

### Test 3: High Bandwidth Test

```bash
# Edit client/client-config.toml:
# bandwidth = "100Mbps"
# connections = 50
# request_size = "100KB"

# Terminal 1: Server
cd server
sudo ./target/release/server --iface eth0

# Terminal 2: Generate traffic
cd client
./target/release/sampler-client --duration 30s

# Observe high bandwidth in dashboard
```

### Test 4: Latency Simulation

```bash
# Generate slow network traffic
cd client
./target/release/sampler-client \
  --bandwidth 100KB/s \
  --latency 500ms \
  --connections 2 \
  --duration 60s
```

## API Usage

### Get Real-time Metrics

```bash
curl http://localhost:3000/api/metrics/realtime | jq
```

Response:
```json
{
  "bandwidth_bps": 10485760,
  "packets_per_sec": 1024,
  "tcp_bandwidth_bps": 10485760,
  "tcp_packets_per_sec": 1024,
  "http_bandwidth_bps": 0,
  "http_packets_per_sec": 0,
  "https_bandwidth_bps": 10485760,
  "https_packets_per_sec": 1024,
  "total_packets": 61440,
  "total_bytes": 62914560
}
```

### Get Summary Statistics

```bash
curl http://localhost:3000/api/stats/summary | jq
```

## Metrics Explained

- **Bandwidth (bps)**: Bits per second (bytes × 8)
- **Packets/sec**: Number of packets processed per second
- **TCP Traffic**: All TCP protocol traffic
- **HTTP Traffic**: TCP packets on port 80
- **HTTPS Traffic**: TCP packets on port 443
- **Protocol %**: Percentage of total bytes by protocol

## Performance

### Server
- **Overhead**: ~1-2% CPU (eBPF runs in kernel)
- **Throughput**: Handles millions of packets/sec
- **Memory**: ~10MB + per-CPU map overhead
- **Compatibility**: XDP native or SKB mode fallback

### Client
- **Max Bandwidth**: Limited by network and target server
- **Connections**: 1-100+ concurrent (configurable)
- **Overhead**: ~5-10% CPU for high loads

## Troubleshooting

### Server Issues

**"Operation not supported"**
- Kernel doesn't support native XDP
- Server automatically falls back to SKB mode
- Solution: Already handled, should work

**"Permission denied"**
- XDP requires root privileges
- Solution: Run with `sudo`

**Interface not found**
- Check: `ip link show`
- Solution: Use correct interface name with `--iface`

### Client Issues

**Connection refused**
- Target URL unreachable
- Solution: Verify target is accessible

**Lower than expected bandwidth**
- Server or network bottleneck
- Solution: Increase `connections`, reduce `latency`

**High error rate**
- Target server overloaded
- Solution: Reduce bandwidth or connections

## Project Structure

```
sampler/
├── server/                  # eBPF monitoring server
│   ├── server/              # Userspace code
│   ├── server-ebpf/         # eBPF kernel programs
│   ├── server-common/       # Shared data structures
│   └── README.md
├── client/                  # Traffic generator
│   ├── src/
│   ├── client-config.toml   # Configuration file
│   └── README.md
└── README.md               # This file
```

## Requirements

- Linux kernel 5.x+ with eBPF support
- Root privileges (server only)
- Rust 1.70+ (stable for client, nightly for server)
- Network interface for monitoring

## License

Dual MIT/Apache-2.0

eBPF code: GPL-2.0 or MIT
