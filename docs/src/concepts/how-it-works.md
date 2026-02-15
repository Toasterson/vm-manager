# How vmctl Works

## State Directory

vmctl stores all VM state under `$XDG_DATA_HOME/vmctl/` (typically `~/.local/share/vmctl/`):

```
~/.local/share/vmctl/
  vms.json              # VM registry (name -> handle mapping)
  images/               # Downloaded image cache
  vms/
    <vm-name>/          # Per-VM working directory
      overlay.qcow2     # Copy-on-write disk overlay
      seed.iso          # Cloud-init NoCloud ISO
      qmp.sock          # QEMU Machine Protocol socket
      console.sock      # Serial console socket
      console.log       # Boot/cloud-init log
      provision.log     # Provisioning output log
      id_ed25519_generated      # Auto-generated SSH private key
      id_ed25519_generated.pub  # Auto-generated SSH public key
      pidfile            # QEMU process PID
```

## QCOW2 Overlays

vmctl never modifies the base image directly. Instead, it creates a QCOW2 copy-on-write overlay on top of the original. This means:

- Multiple VMs can share the same base image.
- The base image stays clean in the cache.
- Destroying a VM just deletes the overlay.

If you specify `disk` in your VMFile, the overlay is resized to that size and the guest filesystem can be grown.

## RouterHypervisor

All hypervisor operations go through a `RouterHypervisor` that dispatches to the appropriate backend based on the platform:

- **Linux** -> `QemuBackend`
- **illumos** -> `PropolisBackend`
- **Testing** -> `NoopBackend`

Each backend implements the same `Hypervisor` trait, so the CLI code is platform-agnostic.

## The Up Flow

When you run `vmctl up`, the following happens for each VM defined in `VMFile.kdl`:

1. **Parse** - Read and validate the VMFile.
2. **Resolve** - Download images (if URL), generate SSH keys (if cloud-init enabled), resolve paths.
3. **Prepare** - Create work directory, QCOW2 overlay, cloud-init seed ISO, allocate MAC address and SSH port.
4. **Start** - Launch QEMU with the correct arguments, wait for QMP socket.
5. **Provision** - Wait for SSH to become available (up to 120 seconds), then run each provisioner in order.

If a VM is already running, `vmctl up` skips it. If it's stopped, it restarts and re-provisions.
