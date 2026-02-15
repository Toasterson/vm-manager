# Prerequisites

vmctl requires several system tools depending on the backend and features you use.

## Linux (QEMU/KVM)

### Required

| Tool | Purpose | Install (Debian/Ubuntu) |
|---|---|---|
| `qemu-system-x86_64` | VM hypervisor | `sudo apt install qemu-system-x86` |
| `qemu-img` | Disk image operations | `sudo apt install qemu-utils` |
| `/dev/kvm` | Hardware virtualization | Kernel module (usually built-in) |

### Cloud-Init ISO Generation (one of)

| Tool | Purpose | Install |
|---|---|---|
| `genisoimage` | ISO 9660 image creation | `sudo apt install genisoimage` |
| `mkisofs` | Alternative ISO tool | `sudo apt install mkisofs` |

Or build with the `pure-iso` feature to avoid needing either.

## Verify Everything

```bash
# QEMU
qemu-system-x86_64 --version

# qemu-img
qemu-img --version

# KVM access
ls -la /dev/kvm

# ISO tools (one of these)
genisoimage --version 2>/dev/null || mkisofs --version 2>/dev/null

# Your user should be in the kvm group
groups | grep -q kvm && echo "kvm: OK" || echo "kvm: add yourself to the kvm group"
```

If `/dev/kvm` is not present, enable KVM in your BIOS/UEFI settings (look for "VT-x" or "AMD-V") and ensure the `kvm` kernel module is loaded:

```bash
sudo modprobe kvm
sudo modprobe kvm_intel  # or kvm_amd
```

## illumos (Propolis)

For the experimental Propolis backend:

- A running `propolis-server` instance
- ZFS pool (default: `rpool`)
- `nebula-vm` zone brand installed
- VNIC networking configured
