#![no_std]

/// Packet statistics collected by XDP
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PacketStats {
    pub total_packets: u64,
    pub total_bytes: u64,
    pub tcp_packets: u64,
    pub tcp_bytes: u64,
    pub tls_packets: u64,    // Port 443
    pub tls_bytes: u64,
    pub http_packets: u64,   // Port 80
    pub http_bytes: u64,
}

/// TCP flow key (5-tuple)
#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FlowKey {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
    pub _padding: [u8; 3],
}

/// Per-flow statistics
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlowStats {
    pub packets: u64,
    pub bytes: u64,
    pub first_seen: u64,  // nanoseconds since boot
    pub last_seen: u64,
}

/// TCP connection metrics from kprobes
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TcpMetrics {
    pub rtt_us: u32,           // Round-trip time in microseconds
    pub retransmits: u32,      // Retransmission count
    pub _padding: [u8; 8],
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for PacketStats {}
#[cfg(feature = "user")]
unsafe impl aya::Pod for FlowKey {}
#[cfg(feature = "user")]
unsafe impl aya::Pod for FlowStats {}
#[cfg(feature = "user")]
unsafe impl aya::Pod for TcpMetrics {}
