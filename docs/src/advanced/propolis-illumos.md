# illumos / Propolis Backend

vmctl includes experimental support for running VMs on illumos using the Propolis hypervisor (bhyve-based).

## Requirements

- illumos-based OS (OmniOS, SmartOS, etc.)
- `propolis-server` installed and runnable
- ZFS pool (default: `rpool`)
- `nebula-vm` zone brand installed

## How It Works

The Propolis backend manages VMs as illumos zones:

1. **Prepare**: Creates a ZFS clone from `{pool}/images/{vm}@latest` to `{pool}/vms/{vm}`.
2. **Start**: Boots the zone with `zoneadm -z {vm} boot`, waits for propolis-server on `127.0.0.1:12400`, then sends the instance spec and run command via REST API.
3. **Stop**: Sends a stop command to propolis-server, then halts the zone.
4. **Destroy**: Stops the VM, uninstalls the zone (`zoneadm uninstall -F`), deletes the zone config (`zonecfg delete -F`), and destroys the ZFS dataset.

## Networking

Uses illumos VNICs for exclusive-IP zone networking:

```kdl
network "vnic" {
    name "vnic0"
}
```

## Limitations

- Suspend/resume not yet implemented.
- Console endpoint (WebSocket) is defined but not fully integrated.
- VNC address not yet exposed.

## Building for illumos

```bash
cargo build --release -p vmctl --target x86_64-unknown-illumos
```
