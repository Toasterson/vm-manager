# SSH and Provisioning API

## SSH Module

Located in `crates/vm-manager/src/ssh.rs`.

### connect

```rust
pub fn connect(ip: &str, port: u16, config: &SshConfig) -> Result<Session>
```

Establishes an SSH connection and authenticates. Supports in-memory PEM keys and file-based keys.

### exec

```rust
pub fn exec(sess: &Session, cmd: &str) -> Result<(String, String, i32)>
```

Executes a command and returns `(stdout, stderr, exit_code)`.

### exec_streaming

```rust
pub fn exec_streaming<W1: Write, W2: Write>(
    sess: &Session,
    cmd: &str,
    stdout_writer: &mut W1,
    stderr_writer: &mut W2,
) -> Result<(String, String, i32)>
```

Executes a command with real-time output streaming. Uses non-blocking I/O with 8KB buffers and 50ms polling interval. Both writes to the provided writers and collects the full output.

### upload

```rust
pub fn upload(sess: &Session, local: &Path, remote: &str) -> Result<()>
```

Uploads a file via SFTP.

### connect_with_retry

```rust
pub async fn connect_with_retry(
    ip: &str,
    port: u16,
    config: &SshConfig,
    timeout: Duration,
) -> Result<Session>
```

Retries connection with exponential backoff (1s to 5s). Runs blocking SSH on `tokio::task::spawn_blocking`.

## Provisioning Module

Located in `crates/vm-manager/src/provision.rs`.

### run_provisions

```rust
pub fn run_provisions(
    sess: &Session,
    provisions: &[ProvisionDef],
    base_dir: &Path,
    vm_name: &str,
    log_dir: Option<&Path>,
) -> Result<()>
```

Runs all provisioners in sequence:

1. **Shell (inline)**: Executes the command via `exec_streaming`.
2. **Shell (script)**: Uploads the script to `/tmp/vmctl-provision-<step>.sh`, makes it executable, runs it.
3. **File**: Uploads via SFTP.

Output is streamed to the terminal and appended to `provision.log` if `log_dir` is provided.

Aborts on the first non-zero exit code with `VmError::ProvisionFailed`.
