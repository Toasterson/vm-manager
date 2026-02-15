# Debugging and Logs

## Log Verbosity

vmctl uses the `tracing` crate with `RUST_LOG` environment variable support:

```bash
# Default (info level)
vmctl up

# Debug logging
RUST_LOG=debug vmctl up

# Trace logging (very verbose)
RUST_LOG=trace vmctl up

# Target specific modules
RUST_LOG=vm_manager::ssh=debug vmctl ssh myvm
```

## VM Logs

### Console Log

The serial console output is captured to `console.log` in the VM's work directory. This includes boot messages and cloud-init output:

```bash
vmctl log myvm --console
```

### Provision Log

Provisioner stdout/stderr is captured to `provision.log`:

```bash
vmctl log myvm --provision
```

### Tail Recent Output

```bash
vmctl log myvm --console --tail 50
```

## Work Directory

Each VM's files are in `~/.local/share/vmctl/vms/<name>/`:

```bash
ls ~/.local/share/vmctl/vms/myvm/
```

Contents:
- `overlay.qcow2` - Disk overlay
- `seed.iso` - Cloud-init ISO
- `console.log` - Serial output
- `provision.log` - Provisioner output
- `qmp.sock` - QMP control socket
- `console.sock` - Console socket
- `pidfile` - QEMU PID
- `id_ed25519_generated` - Auto-generated SSH key
- `id_ed25519_generated.pub` - Public key

## QMP Socket

You can interact with the QEMU Machine Protocol directly for advanced debugging:

```bash
# Using socat
socat - UNIX-CONNECT:~/.local/share/vmctl/vms/myvm/qmp.sock
```

After connecting, send `{"execute": "qmp_capabilities"}` to initialize, then commands like:

```json
{"execute": "query-status"}
{"execute": "query-vnc"}
{"execute": "human-monitor-command", "arguments": {"command-line": "info network"}}
```

## Common Issues

### "QEMU spawn failed"

- Verify `qemu-system-x86_64` is in your PATH.
- Check `/dev/kvm` exists and is accessible.
- Ensure your user is in the `kvm` group.

### "Cloud-init ISO failed"

- Install `genisoimage` or `mkisofs`.
- Or rebuild with `--features vm-manager/pure-iso`.

### "SSH failed"

- Check the console log for cloud-init errors: `vmctl log myvm --console`
- Verify the guest is reachable (check `vmctl status myvm` for SSH port).
- Ensure sshd is running in the guest.
- Try connecting manually: `ssh -p <port> -i <key> user@127.0.0.1`

### "IP discovery timeout" (TAP networking)

- Verify the bridge exists and has DHCP.
- Check `ip neigh show` for the guest's MAC address.
- Ensure the guest has obtained a DHCP lease (check console log).

### VM stuck in "Stopped" state but QEMU still running

- Check `vmctl status myvm` for the PID.
- Verify: `kill -0 <pid>` - if the process is alive, the QMP socket may be stale.
- Destroy and recreate: `vmctl destroy myvm`.
