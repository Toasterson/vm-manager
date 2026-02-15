# Core Types

All types are defined in `crates/vm-manager/src/types.rs` and re-exported from the crate root.

## VmSpec

The input specification for creating a VM.

```rust
pub struct VmSpec {
    pub name: String,
    pub image_path: PathBuf,
    pub vcpus: u16,
    pub memory_mb: u64,
    pub disk_gb: Option<u32>,
    pub network: NetworkConfig,
    pub cloud_init: Option<CloudInitConfig>,
    pub ssh: Option<SshConfig>,
}
```

## VmHandle

A runtime handle to a managed VM. Serializable to JSON for persistence.

```rust
pub struct VmHandle {
    pub id: String,
    pub name: String,
    pub backend: BackendTag,
    pub work_dir: PathBuf,
    pub overlay_path: Option<PathBuf>,
    pub seed_iso_path: Option<PathBuf>,
    pub pid: Option<u32>,
    pub qmp_socket: Option<PathBuf>,
    pub console_socket: Option<PathBuf>,
    pub vnc_addr: Option<String>,
    pub vcpus: u16,            // default: 1
    pub memory_mb: u64,        // default: 1024
    pub disk_gb: Option<u32>,
    pub network: NetworkConfig,
    pub ssh_host_port: Option<u16>,
    pub mac_addr: Option<String>,
}
```

All optional fields default to `None` and numeric fields have sensible defaults for backward-compatible deserialization.

## VmState

```rust
pub enum VmState {
    Preparing,
    Prepared,
    Running,
    Stopped,
    Failed,
    Destroyed,
}
```

Implements `Display` with lowercase names.

## NetworkConfig

```rust
pub enum NetworkConfig {
    Tap { bridge: String },
    User,                    // default
    Vnic { name: String },
    None,
}
```

Serialized with `#[serde(tag = "type")]` for clean JSON representation.

## CloudInitConfig

```rust
pub struct CloudInitConfig {
    pub user_data: Vec<u8>,
    pub instance_id: Option<String>,
    pub hostname: Option<String>,
}
```

`user_data` is the raw cloud-config YAML content.

## SshConfig

```rust
pub struct SshConfig {
    pub user: String,
    pub public_key: Option<String>,
    pub private_key_path: Option<PathBuf>,
    pub private_key_pem: Option<String>,
}
```

Supports both file-based keys (`private_key_path`) and in-memory keys (`private_key_pem`).

## BackendTag

```rust
pub enum BackendTag {
    Noop,
    Qemu,
    Propolis,
}
```

Serialized as lowercase strings. Implements `Display`.
