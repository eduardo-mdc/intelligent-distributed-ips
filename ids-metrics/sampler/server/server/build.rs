use aya_build::{Toolchain, Package};

fn main() -> anyhow::Result<()> {
    aya_build::build_ebpf(
        [Package {
            name: "server-ebpf",
            root_dir: "../server-ebpf".into(),
        }],
        Toolchain::default(),
    )
}
