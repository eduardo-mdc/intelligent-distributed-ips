use anyhow::{Context, Result};
use byte_unit::Byte;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TrafficConfig {
    pub target_url: String,
    pub target_bandwidth_bps: u64,
    pub connections: usize,
    pub total_bytes: u64, // 0 = infinite
    pub request_delay: Duration,
    pub run_duration: Duration, // 0 = infinite
    pub protocol: Protocol,
    pub request_size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Http,
    Https,
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFile {
    #[serde(default)]
    target: TargetConfig,
    #[serde(default)]
    traffic: TrafficConfigFile,
    #[serde(default)]
    limits: LimitsConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct TargetConfig {
    #[serde(default = "default_url")]
    url: String,
    #[serde(default = "default_protocol")]
    protocol: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrafficConfigFile {
    #[serde(default = "default_bandwidth")]
    bandwidth: String,
    #[serde(default = "default_connections")]
    connections: usize,
    #[serde(default = "default_request_size")]
    request_size: String,
    #[serde(default = "default_latency")]
    latency: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LimitsConfig {
    #[serde(default = "default_duration")]
    duration: String,
    #[serde(default = "default_total_size")]
    total_size: String,
}

// Default values
fn default_url() -> String {
    "http://httpbin.org".to_string()
}
fn default_protocol() -> String {
    "http".to_string()
}
fn default_bandwidth() -> String {
    "1MB/s".to_string()
}
fn default_connections() -> usize {
    10
}
fn default_request_size() -> String {
    "1KB".to_string()
}
fn default_latency() -> String {
    "0ms".to_string()
}
fn default_duration() -> String {
    "0s".to_string()
}
fn default_total_size() -> String {
    "0".to_string()
}

impl Default for TargetConfig {
    fn default() -> Self {
        Self {
            url: default_url(),
            protocol: default_protocol(),
        }
    }
}

impl Default for TrafficConfigFile {
    fn default() -> Self {
        Self {
            bandwidth: default_bandwidth(),
            connections: default_connections(),
            request_size: default_request_size(),
            latency: default_latency(),
        }
    }
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            duration: default_duration(),
            total_size: default_total_size(),
        }
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            target: TargetConfig::default(),
            traffic: TrafficConfigFile::default(),
            limits: LimitsConfig::default(),
        }
    }
}

impl TrafficConfig {
    /// Load configuration with priority: defaults < config file < CLI args
    pub fn load(cli: crate::Cli) -> Result<Self> {
        // Start with defaults
        let mut config_file = ConfigFile::default();

        // Try to load config file
        if let Some(config_path) = &cli.config {
            config_file = Self::load_config_file(config_path)?;
        } else {
            // Try default locations
            for path in Self::default_config_paths() {
                if path.exists() {
                    config_file = Self::load_config_file(&path.to_string_lossy())?;
                    log::info!("Loaded config from: {}", path.display());
                    break;
                }
            }
        }

        // Override with CLI args
        if let Some(target) = cli.target {
            config_file.target.url = target;
        }
        if let Some(protocol) = cli.protocol {
            config_file.target.protocol = protocol;
        }
        if let Some(bandwidth) = cli.bandwidth {
            config_file.traffic.bandwidth = bandwidth;
        }
        if let Some(connections) = cli.connections {
            config_file.traffic.connections = connections;
        }
        if let Some(request_size) = cli.request_size {
            config_file.traffic.request_size = request_size;
        }
        if let Some(latency) = cli.latency {
            config_file.traffic.latency = latency;
        }
        if let Some(duration) = cli.duration {
            config_file.limits.duration = duration;
        }
        if let Some(total_size) = cli.total_size {
            config_file.limits.total_size = total_size;
        }

        // Convert to TrafficConfig
        Self::from_config_file(config_file)
    }

    fn load_config_file(path: &str) -> Result<ConfigFile> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;
        toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path))
    }

    fn default_config_paths() -> Vec<PathBuf> {
        vec![
            PathBuf::from("client-config.toml"),
            PathBuf::from("config.toml"),
            dirs::config_dir()
                .map(|d| d.join("sampler-client/config.toml"))
                .unwrap_or_default(),
        ]
    }

    fn from_config_file(config: ConfigFile) -> Result<Self> {
        // Parse bandwidth
        let target_bandwidth_bps = Self::parse_bandwidth(&config.traffic.bandwidth)?;

        // Parse total size
        let total_bytes = if config.limits.total_size == "0" {
            0
        } else {
            Self::parse_size(&config.limits.total_size)?
        };

        // Parse latency
        let request_delay = humantime::parse_duration(&config.traffic.latency)
            .context("Invalid latency format")?;

        // Parse duration
        let run_duration = humantime::parse_duration(&config.limits.duration)
            .context("Invalid duration format")?;

        // Parse protocol
        let protocol = match config.target.protocol.to_lowercase().as_str() {
            "http" => Protocol::Http,
            "https" => Protocol::Https,
            _ => anyhow::bail!("Invalid protocol: {}", config.target.protocol),
        };

        // Parse request size
        let request_size = Self::parse_size(&config.traffic.request_size)? as usize;

        Ok(Self {
            target_url: config.target.url,
            target_bandwidth_bps,
            connections: config.traffic.connections,
            total_bytes,
            request_delay,
            run_duration,
            protocol,
            request_size,
        })
    }

    fn parse_bandwidth(s: &str) -> Result<u64> {
        let s = s.to_lowercase();

        if s.ends_with("bps") {
            let num = s.trim_end_matches("bps");
            let byte = Byte::parse_str(num, true)
                .context("Invalid bandwidth format")?;
            Ok(byte.as_u64())
        } else if s.ends_with("/s") {
            let num = s.trim_end_matches("/s");
            let byte = Byte::parse_str(num, true)
                .context("Invalid bandwidth format")?;
            Ok(byte.as_u64() * 8)
        } else {
            anyhow::bail!("Bandwidth must end with 'bps' or '/s'")
        }
    }

    fn parse_size(s: &str) -> Result<u64> {
        let byte = Byte::parse_str(s, true)
            .context("Invalid size format")?;
        Ok(byte.as_u64())
    }

    pub fn bytes_per_second(&self) -> u64 {
        self.target_bandwidth_bps / 8
    }

    pub fn requests_per_second(&self) -> f64 {
        if self.request_size == 0 {
            return 0.0;
        }
        self.bytes_per_second() as f64 / self.request_size as f64
    }
}

// Helper for home directory (cross-platform)
mod dirs {
    use std::path::PathBuf;

    pub fn config_dir() -> Option<PathBuf> {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
    }
}
