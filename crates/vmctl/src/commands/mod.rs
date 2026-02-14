pub mod console;
pub mod create;
pub mod destroy;
pub mod down;
pub mod image;
pub mod list;
pub mod provision_cmd;
pub mod reload;
pub mod ssh;
pub mod start;
pub mod state;
pub mod status;
pub mod stop;
pub mod up;

use clap::{Parser, Subcommand};
use miette::Result;
use vm_manager::{NetworkConfig, VmHandle};

#[derive(Parser)]
#[command(name = "vmctl", about = "Manage virtual machines", version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new VM (and optionally start it)
    Create(create::CreateArgs),
    /// Start an existing VM
    Start(start::StartArgs),
    /// Stop a running VM
    Stop(stop::StopArgs),
    /// Destroy a VM and clean up all resources
    Destroy(destroy::DestroyArgs),
    /// List all VMs
    List(list::ListArgs),
    /// Show VM status
    Status(status::StatusArgs),
    /// Attach to a VM's serial console
    Console(console::ConsoleArgs),
    /// SSH into a VM
    Ssh(ssh::SshArgs),
    /// Suspend a running VM (pause vCPUs)
    Suspend(start::SuspendArgs),
    /// Resume a suspended VM
    Resume(start::ResumeArgs),
    /// Manage VM images
    Image(image::ImageCommand),
    /// Bring up VMs defined in VMFile.kdl
    Up(up::UpArgs),
    /// Bring down VMs defined in VMFile.kdl
    Down(down::DownArgs),
    /// Destroy and recreate VMs defined in VMFile.kdl
    Reload(reload::ReloadArgs),
    /// Re-run provisioners on running VMs from VMFile.kdl
    Provision(provision_cmd::ProvisionArgs),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Command::Create(args) => create::run(args).await,
            Command::Start(args) => start::run_start(args).await,
            Command::Stop(args) => stop::run(args).await,
            Command::Destroy(args) => destroy::run(args).await,
            Command::List(args) => list::run(args).await,
            Command::Status(args) => status::run(args).await,
            Command::Console(args) => console::run(args).await,
            Command::Ssh(args) => ssh::run(args).await,
            Command::Suspend(args) => start::run_suspend(args).await,
            Command::Resume(args) => start::run_resume(args).await,
            Command::Image(args) => image::run(args).await,
            Command::Up(args) => up::run(args).await,
            Command::Down(args) => down::run(args).await,
            Command::Reload(args) => reload::run(args).await,
            Command::Provision(args) => provision_cmd::run(args).await,
        }
    }
}

/// Determine the SSH port for a VM handle: use the forwarded host port for user-mode networking,
/// or 22 for all other network types.
fn ssh_port_for_handle(handle: &VmHandle) -> u16 {
    match handle.network {
        NetworkConfig::User => handle.ssh_host_port.unwrap_or(22),
        _ => 22,
    }
}
