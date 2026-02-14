use clap::Args;
use miette::{IntoDiagnostic, Result};
use vm_manager::{Hypervisor, NetworkConfig, RouterHypervisor};

use super::state;

#[derive(Args)]
pub struct StatusArgs {
    /// VM name
    name: String,
}

pub async fn run(args: StatusArgs) -> Result<()> {
    let store = state::load_store().await?;
    let handle = store
        .get(&args.name)
        .ok_or_else(|| miette::miette!("VM '{}' not found", args.name))?;

    let hv = RouterHypervisor::new(None, None);
    let state = hv.state(handle).await.into_diagnostic()?;

    println!("Name:    {}", handle.name);
    println!("ID:      {}", handle.id);
    println!("Backend: {}", handle.backend);
    println!("State:   {}", state);
    println!("vCPUs:   {}", handle.vcpus);
    println!("Memory:  {} MB", handle.memory_mb);
    if let Some(disk) = handle.disk_gb {
        println!("Disk:    {} GB", disk);
    }
    println!("Network: {}", format_network(&handle.network));
    println!("WorkDir: {}", handle.work_dir.display());

    if let Some(ref overlay) = handle.overlay_path {
        println!("Overlay: {}", overlay.display());
    }
    if let Some(ref seed) = handle.seed_iso_path {
        println!("Seed:    {}", seed.display());
    }
    if let Some(pid) = handle.pid {
        println!("PID:     {}", pid);
    }
    if let Some(ref vnc) = handle.vnc_addr {
        println!("VNC:     {}", vnc);
    }
    if let Some(port) = handle.ssh_host_port {
        println!("SSH:     127.0.0.1:{}", port);
    }
    if let Some(ref mac) = handle.mac_addr {
        println!("MAC:     {}", mac);
    }

    Ok(())
}

fn format_network(net: &NetworkConfig) -> String {
    match net {
        NetworkConfig::Tap { bridge } => format!("tap (bridge: {bridge})"),
        NetworkConfig::User => "user (SLIRP)".into(),
        NetworkConfig::Vnic { name } => format!("vnic ({name})"),
        NetworkConfig::None => "none".into(),
    }
}
