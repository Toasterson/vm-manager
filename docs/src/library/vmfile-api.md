# VMFile Parsing API

The VMFile module parses and resolves `VMFile.kdl` configuration files. Located in `crates/vm-manager/src/vmfile.rs`.

## Types

### VmFile

```rust
pub struct VmFile {
    pub base_dir: PathBuf,
    pub vms: Vec<VmDef>,
}
```

### VmDef

```rust
pub struct VmDef {
    pub name: String,
    pub image: ImageSource,
    pub vcpus: u16,
    pub memory_mb: u64,
    pub disk_gb: Option<u32>,
    pub network: NetworkDef,
    pub cloud_init: Option<CloudInitDef>,
    pub ssh: Option<SshDef>,
    pub provisions: Vec<ProvisionDef>,
}
```

### ImageSource

```rust
pub enum ImageSource {
    Local(String),
    Url(String),
}
```

### ProvisionDef

```rust
pub enum ProvisionDef {
    Shell(ShellProvision),
    File(FileProvision),
}

pub struct ShellProvision {
    pub inline: Option<String>,
    pub script: Option<String>,
}

pub struct FileProvision {
    pub source: String,
    pub destination: String,
}
```

## Functions

### discover

```rust
pub fn discover(explicit: Option<&Path>) -> Result<PathBuf>
```

Finds the VMFile. If `explicit` is provided, uses that path. Otherwise, looks for `VMFile.kdl` in the current directory.

### parse

```rust
pub fn parse(path: &Path) -> Result<VmFile>
```

Parses a VMFile.kdl into a `VmFile` struct. Validates:
- At least one `vm` block.
- No duplicate VM names.
- Each VM has a valid image source.
- Provisioner blocks are well-formed.

### resolve

```rust
pub async fn resolve(def: &VmDef, base_dir: &Path) -> Result<VmSpec>
```

Converts a `VmDef` into a `VmSpec` ready for the hypervisor:
- Downloads images from URLs.
- Resolves local image paths.
- Generates Ed25519 SSH keypairs if needed.
- Reads cloud-init user-data files.
- Resolves all relative paths against `base_dir`.

### Utility Functions

```rust
pub fn expand_tilde(s: &str) -> PathBuf
```

Expands `~` to the user's home directory.

```rust
pub fn resolve_path(raw: &str, base_dir: &Path) -> PathBuf
```

Expands tilde and makes relative paths absolute against `base_dir`.

```rust
pub fn generate_ssh_keypair(vm_name: &str) -> Result<(String, String)>
```

Generates an Ed25519 keypair. Returns `(public_key_openssh, private_key_pem)`.
