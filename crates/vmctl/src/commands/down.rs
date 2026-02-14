use std::path::PathBuf;
use std::time::Duration;

use clap::Args;
use miette::{IntoDiagnostic, Result};
use vm_manager::{Hypervisor, RouterHypervisor};

use super::state;

#[derive(Args)]
pub struct DownArgs {
    /// Path to VMFile.kdl
    #[arg(long)]
    file: Option<PathBuf>,

    /// Only bring down a specific VM by name
    #[arg(long)]
    name: Option<String>,

    /// Destroy VMs instead of just stopping them
    #[arg(long)]
    destroy: bool,
}

pub async fn run(args: DownArgs) -> Result<()> {
    let path = vm_manager::vmfile::discover(args.file.as_deref()).into_diagnostic()?;
    let vmfile = vm_manager::vmfile::parse(&path).into_diagnostic()?;

    let mut store = state::load_store().await?;
    let hv = RouterHypervisor::new(None, None);

    for def in &vmfile.vms {
        if let Some(ref filter) = args.name {
            if &def.name != filter {
                continue;
            }
        }

        if let Some(handle) = store.get(&def.name).cloned() {
            if args.destroy {
                store.remove(&def.name);
                hv.destroy(handle).await.into_diagnostic()?;
                state::save_store(&store).await?;
                println!("VM '{}' destroyed", def.name);
            } else {
                let updated = hv
                    .stop(&handle, Duration::from_secs(30))
                    .await
                    .into_diagnostic()?;
                store.insert(def.name.clone(), updated);
                state::save_store(&store).await?;
                println!("VM '{}' stopped", def.name);
            }
        } else {
            println!("VM '{}' not found in store — skipping", def.name);
        }
    }

    Ok(())
}
