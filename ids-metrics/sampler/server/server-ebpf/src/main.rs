#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::PerCpuArray,
    programs::XdpContext,
};
use core::mem;
use server_common::PacketStats;

// Network protocol constants
const ETH_P_IP: u16 = 0x0800;
const IPPROTO_TCP: u8 = 6;

// Port constants
const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;

#[map]
static PACKET_STATS: PerCpuArray<PacketStats> = PerCpuArray::with_max_entries(1, 0);

#[xdp]
pub fn server(ctx: XdpContext) -> u32 {
    match try_server(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_server(ctx: XdpContext) -> Result<u32, u32> {
    let ethhdr_len = mem::size_of::<EthHdr>();
    let eth_hdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };

    let ether_type = unsafe { (*eth_hdr).ether_type };
    if ether_type != u16::to_be(ETH_P_IP) {
        return Ok(xdp_action::XDP_PASS);
    }

    let ip_hdr: *const IpHdr = unsafe { ptr_at(&ctx, ethhdr_len)? };

    let protocol = unsafe { (*ip_hdr).protocol };
    let total_len = unsafe { u16::from_be((*ip_hdr).tot_len) } as u64;

    // Update stats
    if let Some(stats) = PACKET_STATS.get_ptr_mut(0) {
        let stats = unsafe { &mut *stats };
        stats.total_packets += 1;
        stats.total_bytes += total_len;

        if protocol == IPPROTO_TCP {
            stats.tcp_packets += 1;
            stats.tcp_bytes += total_len;

            // Parse TCP header to get ports
            let ip_hdr_len = ((unsafe { (*ip_hdr).ihl } & 0x0F) * 4) as usize;
            let tcphdr_offset = ethhdr_len + ip_hdr_len;

            if let Ok(tcp_hdr) = unsafe { ptr_at::<TcpHdr>(&ctx, tcphdr_offset) } {
                let dst_port = u16::from_be(unsafe { (*tcp_hdr).dest });
                let src_port = u16::from_be(unsafe { (*tcp_hdr).source });

                if dst_port == HTTPS_PORT || src_port == HTTPS_PORT {
                    stats.tls_packets += 1;
                    stats.tls_bytes += total_len;
                } else if dst_port == HTTP_PORT || src_port == HTTP_PORT {
                    stats.http_packets += 1;
                    stats.http_bytes += total_len;
                }
            }
        }
    }

    Ok(xdp_action::XDP_PASS)
}

#[repr(C)]
struct EthHdr {
    dst: [u8; 6],
    src: [u8; 6],
    ether_type: u16,
}

#[repr(C)]
struct IpHdr {
    ihl: u8,
    tos: u8,
    tot_len: u16,
    id: u16,
    frag_off: u16,
    ttl: u8,
    protocol: u8,
    check: u16,
    saddr: u32,
    daddr: u32,
}

#[repr(C)]
struct TcpHdr {
    source: u16,
    dest: u16,
    seq: u32,
    ack_seq: u32,
    _bitfield: u16,
    window: u16,
    check: u16,
    urg_ptr: u16,
}

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, u32> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(1);
    }

    Ok((start + offset) as *const T)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
