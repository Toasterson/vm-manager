# Crate Structure

## Workspace Layout

```text
vm-manager/
  Cargo.toml              # Workspace root
  crates/
    vm-manager/            # Library crate
      Cargo.toml
      src/
        lib.rs             # Re-exports
        traits.rs          # Hypervisor trait, ConsoleEndpoint
        types.rs           # VmSpec, VmHandle, VmState, NetworkConfig, etc.
        error.rs           # VmError with miette diagnostics
        vmfile.rs          # VMFile.kdl parser and resolver
        image.rs           # ImageManager (download, cache, overlay)
        ssh.rs             # SSH connect, exec, streaming, upload
        provision.rs       # Provisioner runner
        cloudinit.rs       # NoCloud seed ISO generation
        backends/
          mod.rs           # RouterHypervisor
          qemu.rs          # QEMU/KVM backend (Linux)
          qmp.rs           # QMP client
          propolis.rs       # Propolis/bhyve backend (illumos)
          noop.rs          # No-op backend (testing)
    vmctl/                 # CLI binary crate
      Cargo.toml
      src/
        main.rs            # CLI entry point, clap App
        commands/
          create.rs        # vmctl create
          start.rs         # vmctl start, suspend, resume
          stop.rs          # vmctl stop
          destroy.rs       # vmctl destroy
          list.rs          # vmctl list
          status.rs        # vmctl status
          console.rs       # vmctl console
          ssh.rs           # vmctl ssh
          image.rs         # vmctl image (pull, list, inspect)
          up.rs            # vmctl up
          down.rs          # vmctl down
          reload.rs        # vmctl reload
          provision_cmd.rs # vmctl provision
          log.rs           # vmctl log
```

## vm-manager Crate

The library crate. Contains all business logic and can be used as a dependency by other Rust projects.

**Public re-exports from `lib.rs`:**
- `RouterHypervisor` (from `backends`)
- `Hypervisor`, `ConsoleEndpoint` (from `traits`)
- `VmError`, `Result` (from `error`)
- All types from `types`: `BackendTag`, `VmSpec`, `VmHandle`, `VmState`, `NetworkConfig`, `CloudInitConfig`, `SshConfig`

## vmctl Crate

The CLI binary. Depends on `vm-manager` and adds:
- Clap-based argument parsing
- Store persistence (`vms.json`)
- Terminal I/O (console bridging, log display)
- VMFile discovery and command dispatch
