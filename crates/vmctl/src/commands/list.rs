use clap::Args;
use miette::Result;
use vm_manager::NetworkConfig;

use super::state;

#[derive(Args)]
pub struct ListArgs;

pub async fn run(_args: ListArgs) -> Result<()> {
    let store = state::load_store().await?;

    if store.is_empty() {
        println!("No VMs found.");
        return Ok(());
    }

    println!(
        "{:<16} {:<8} {:>5} {:>6} {:<10} {:<8} SSH",
        "NAME", "BACKEND", "VCPUS", "MEM", "NETWORK", "PID"
    );
    println!("{}", "-".repeat(72));

    let mut entries: Vec<_> = store.iter().collect();
    entries.sort_by_key(|(name, _)| (*name).clone());

    for (name, handle) in entries {
        let net = match &handle.network {
            NetworkConfig::Tap { .. } => "tap",
            NetworkConfig::User => "user",
            NetworkConfig::Vnic { .. } => "vnic",
            NetworkConfig::None => "none",
        };
        let pid = handle
            .pid
            .map(|p| p.to_string())
            .unwrap_or_else(|| "-".into());
        let ssh = handle
            .ssh_host_port
            .map(|p| format!(":{p}"))
            .unwrap_or_else(|| "-".into());

        println!(
            "{:<16} {:<8} {:>5} {:>4}MB {:<10} {:<8} {}",
            name, handle.backend, handle.vcpus, handle.memory_mb, net, pid, ssh
        );
    }

    Ok(())
}
