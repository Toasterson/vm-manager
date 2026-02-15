# SSH Subsystem

## Library

vmctl uses the `ssh2` crate (Rust bindings to libssh2) for SSH operations. The SSH module is at `crates/vm-manager/src/ssh.rs`.

## Core Functions

### connect

Establishes a TCP connection and authenticates via public key.

Supports two authentication modes:
- **In-memory PEM**: Private key stored as a string (used for auto-generated keys).
- **File path**: Reads key from disk.

### exec

Executes a command and collects the full stdout/stderr output. Blocking.

### exec_streaming

Executes a command and streams stdout/stderr in real-time to provided writers. Uses non-blocking I/O:

1. Opens a channel and calls `exec()`.
2. Switches the session to non-blocking mode.
3. Polls stdout and stderr in a loop with 8KB buffers.
4. Flushes output after each read.
5. Sleeps 50ms when no data is available.
6. Switches back to blocking mode to read the exit status.

This is used by the provisioner to show build output live.

### upload

Transfers a file to the guest via SFTP. Creates the SFTP subsystem, opens a remote file, and writes the local file contents.

### connect_with_retry

Attempts to connect repeatedly until a timeout (typically 120 seconds for provisioning, 30 seconds for `vmctl ssh`). Uses exponential backoff starting at 1 second, capped at 5 seconds. Runs the blocking connect on `tokio::task::spawn_blocking`.

## Why Not Native SSH?

libssh2 is used for programmatic operations (provisioning, connectivity checks) because it can be controlled from Rust code. For interactive sessions (`vmctl ssh`), vmctl hands off to the system `ssh` binary for proper terminal handling (PTY allocation, signal forwarding, etc.).
