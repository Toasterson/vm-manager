# Using vm-manager as a Crate

The `vm-manager` library can be used as a Rust dependency for building custom VM management tools.

## Add the Dependency

```toml
[dependencies]
vm-manager = { path = "crates/vm-manager" }
# or from a git repository:
# vm-manager = { git = "https://github.com/user/vm-manager.git" }
```

## Re-Exports

The crate root re-exports the most commonly used types:

```rust
use vm_manager::{
    // Hypervisor abstraction
    Hypervisor, ConsoleEndpoint, RouterHypervisor,
    // Error handling
    VmError, Result,
    // Core types
    BackendTag, VmSpec, VmHandle, VmState,
    NetworkConfig, CloudInitConfig, SshConfig,
};
```

## Minimal Example

```rust
use vm_manager::{RouterHypervisor, Hypervisor, VmSpec, NetworkConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> vm_manager::Result<()> {
    // Create a hypervisor (platform-detected)
    let hyp = RouterHypervisor::new(None, "rpool".into());

    // Define a VM
    let spec = VmSpec {
        name: "example".into(),
        image_path: "/path/to/image.qcow2".into(),
        vcpus: 2,
        memory_mb: 2048,
        disk_gb: Some(20),
        network: NetworkConfig::User,
        cloud_init: None,
        ssh: None,
    };

    // Lifecycle
    let handle = hyp.prepare(&spec).await?;
    let handle = hyp.start(&handle).await?;

    // ... use the VM ...

    hyp.stop(&handle, Duration::from_secs(30)).await?;
    hyp.destroy(handle).await?;

    Ok(())
}
```

## Feature Flags

| Feature | Effect |
|---|---|
| `pure-iso` | Use pure-Rust ISO generation instead of `genisoimage`/`mkisofs` |
