# Error Handling

## Approach

vm-manager uses [miette](https://docs.rs/miette) for rich diagnostic error reporting. Every error variant includes:

- A human-readable message.
- A diagnostic code (e.g., `vm_manager::qemu::spawn_failed`).
- A `help` message telling the user what to do.

Errors are defined with `#[derive(thiserror::Error, miette::Diagnostic)]`.

## Error Variants

| Code | Trigger | Help |
|---|---|---|
| `vm_manager::qemu::spawn_failed` | QEMU process failed to start | Ensure `qemu-system-x86_64` is installed, in PATH, and KVM is available (`/dev/kvm`) |
| `vm_manager::qemu::qmp_connect_failed` | Can't connect to QMP socket | QEMU may have crashed before QMP socket ready; check work directory logs |
| `vm_manager::qemu::qmp_command_failed` | QMP command returned an error | (varies) |
| `vm_manager::image::overlay_creation_failed` | QCOW2 overlay creation failed | Ensure `qemu-img` is installed and base image exists and is readable |
| `vm_manager::network::ip_discovery_timeout` | Guest IP not found | Guest may not have DHCP lease; check network config and cloud-init |
| `vm_manager::propolis::unreachable` | Can't reach propolis-server | Ensure propolis-server is running and listening on expected address |
| `vm_manager::cloudinit::iso_failed` | Seed ISO generation failed | Ensure `genisoimage` or `mkisofs` installed, or enable `pure-iso` feature |
| `vm_manager::ssh::failed` | SSH connection or command failed | Check SSH key, guest reachability, and sshd running |
| `vm_manager::ssh::keygen_failed` | Ed25519 key generation failed | Internal error; please report it |
| `vm_manager::image::download_failed` | Image download failed | Check network connectivity and URL correctness |
| `vm_manager::image::format_detection_failed` | Can't detect image format | Ensure `qemu-img` installed and file is valid disk image |
| `vm_manager::image::conversion_failed` | Image format conversion failed | Ensure `qemu-img` installed and sufficient disk space |
| `vm_manager::vm::not_found` | VM not in store | Run `vmctl list` to see available VMs |
| `vm_manager::vm::invalid_state` | Operation invalid for current state | (varies) |
| `vm_manager::backend::not_available` | Backend not supported on platform | Backend not supported on current platform |
| `vm_manager::vmfile::not_found` | VMFile.kdl not found | Create VMFile.kdl in current directory or specify path with `--file` |
| `vm_manager::vmfile::parse_failed` | KDL syntax error | Check VMFile.kdl syntax; see https://kdl.dev |
| `vm_manager::vmfile::validation` | VMFile validation error | (custom hint per error) |
| `vm_manager::provision::failed` | Provisioner step failed | Check provisioner config and VM SSH reachability |
| `vm_manager::io` | General I/O error | (transparent) |

## Type Alias

The library defines `pub type Result<T> = std::result::Result<T, VmError>` for convenience. CLI commands return `miette::Result<()>` for rich terminal output.
