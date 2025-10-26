# Traffic Generator Client

Configurable HTTP/HTTPS traffic generator for testing network metrics monitoring systems.

## Features

- **Configurable bandwidth** - Target specific throughput rates
- **Concurrent connections** - Multiple parallel workers
- **Protocol support** - HTTP and HTTPS traffic
- **Latency simulation** - Add artificial delays between requests
- **Config file + CLI** - TOML config with CLI overrides
- **Real-time stats** - Live bandwidth and request metrics

## Installation

```bash
cargo build --release
```

Binary location: `target/release/sampler-client`

## Configuration

### Config File (Recommended)

Create `client-config.toml` in the same directory:

```toml
[target]
url = "http://httpbin.org"
protocol = "http"

[traffic]
bandwidth = "10Mbps"      # Target bandwidth
connections = 10          # Concurrent connections
request_size = "1KB"      # Size per request
latency = "0ms"           # Delay between requests

[limits]
duration = "60s"          # Run for 60 seconds (0 = infinite)
total_size = "0"          # Total data to send (0 = infinite)
```

### Config File Locations

The client searches for config files in this order:
1. `./client-config.toml` (current directory)
2. `./config.toml`
3. `~/.config/sampler-client/config.toml`
4. Custom path via `--config` flag

### CLI Arguments (Override Config)

All config values can be overridden via CLI:

```bash
./sampler-client [OPTIONS]

Options:
  -f, --config <PATH>           Config file path
  -t, --target <URL>            Target URL
  -b, --bandwidth <RATE>        Bandwidth (e.g., 1MB/s, 10Mbps)
  -c, --connections <N>         Concurrent connections
  -r, --request-size <SIZE>     Request size (e.g., 1KB, 10KB)
  -l, --latency <DURATION>      Request delay (e.g., 100ms, 1s)
  -d, --duration <DURATION>     Run duration (e.g., 30s, 5m)
  -s, --total-size <SIZE>       Total data to send (e.g., 100MB)
  -p, --protocol <PROTOCOL>     Protocol (http|https)
```
## How It Works

1 **Worker Spawning**:
   - Creates N concurrent tokio tasks (based on `connections`)
   - Each worker sends HTTP POST requests independently

2 **Rate Limiting**:
   - Calculates required delay between requests to achieve target bandwidth
   - Uses tokio interval timers for precise timing

3 **Request Generation**:
   - Creates random payloads of specified size
   - Sends POST requests to target URL
   - Tracks bytes sent and errors

4 **Statistics**:
   - Atomic counters for thread-safe tracking
   - Prints delta stats every second
   - Shows final summary on exit

## Architecture

```
client/
├── src/
│   ├── main.rs           # CLI parsing, config loading
│   ├── config.rs         # Config file + CLI merging
│   ├── stats.rs          # Statistics tracking
│   └── generator/
│       ├── mod.rs        # Traffic generator orchestration
│       └── worker.rs     # Individual HTTP request workers
└── client-config.toml    # Example config file
```

## Performance Tips

- **High bandwidth** (>100 Mbps): Increase `connections` to 20-50
- **Low latency**: Keep `latency` at 0ms, increase `connections`
- **Sustained load**: Use `duration` instead of `total_size`
- **Large requests**: Increase `request_size` to reduce overhead


## License

Dual MIT/Apache-2.0
