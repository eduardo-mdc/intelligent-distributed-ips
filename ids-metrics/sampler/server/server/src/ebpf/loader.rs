use anyhow::{Context, Result};
use aya::programs::{Xdp, XdpFlags};
use aya::Ebpf;
use log::{debug, warn};
use tokio::io::unix::AsyncFd;

/// Manages eBPF program lifecycle
pub struct EbpfManager {
    pub ebpf: Ebpf,
    _interface: String,
}

impl EbpfManager {
    /// Load and attach eBPF program to the specified network interface
    pub async fn new(interface: &str) -> Result<Self> {
        // Bump memlock rlimit for older kernels
        Self::increase_memlock_rlimit();

        // Load eBPF object file
        let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
            env!("OUT_DIR"),
            "/server"
        )))?;

        // Initialize eBPF logger
        Self::init_ebpf_logger(&mut ebpf)?;

        // Attach XDP program
        Self::attach_xdp_program(&mut ebpf, interface)?;

        Ok(Self {
            ebpf,
            _interface: interface.to_string(),
        })
    }

    /// Increase memlock rlimit (needed for older kernels)
    fn increase_memlock_rlimit() {
        let rlim = libc::rlimit {
            rlim_cur: libc::RLIM_INFINITY,
            rlim_max: libc::RLIM_INFINITY,
        };
        let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
        if ret != 0 {
            debug!("Failed to increase memlock rlimit, ret: {}", ret);
        }
    }

    /// Initialize eBPF logging
    fn init_ebpf_logger(ebpf: &mut Ebpf) -> Result<()> {
        match aya_log::EbpfLogger::init(ebpf) {
            Err(e) => {
                warn!("Failed to initialize eBPF logger: {}", e);
            }
            Ok(logger) => {
                let mut logger = AsyncFd::with_interest(logger, tokio::io::Interest::READABLE)?;
                tokio::task::spawn(async move {
                    loop {
                        let mut guard = logger.readable_mut().await.unwrap();
                        guard.get_inner_mut().flush();
                        guard.clear_ready();
                    }
                });
            }
        }
        Ok(())
    }

    /// Attach XDP program to network interface
    fn attach_xdp_program(ebpf: &mut Ebpf, interface: &str) -> Result<()> {
        let program: &mut Xdp = ebpf
            .program_mut("server")
            .context("XDP program 'server' not found")?
            .try_into()?;

        program.load()?;

        // Try native mode first, fall back to SKB mode
        let result = program.attach(interface, XdpFlags::default());
        if result.is_err() {
            warn!("Native XDP mode failed, falling back to SKB mode");
            program.attach(interface, XdpFlags::SKB_MODE)
                .context("Failed to attach XDP program even in SKB mode")?;
        } else {
            result?;
        }

        Ok(())
    }

    /// Get reference to the eBPF object
    pub fn ebpf(&self) -> &Ebpf {
        &self.ebpf
    }
}
