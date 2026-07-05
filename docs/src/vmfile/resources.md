# Resources

Resource nodes control the VM's CPU, memory, and disk allocation.

## vcpus

```kdl
vcpus 2
```

Number of virtual CPUs. Must be greater than 0.

**Default:** `1`

## memory

```kdl
memory 2048
```

Memory in megabytes. Must be greater than 0.

**Default:** `1024` (1 GB)

## disk

```kdl
disk 20
```

Disk size in gigabytes. When specified, the QCOW2 overlay is created with this size, allowing the guest to use more space than the base image provides. Most cloud images auto-grow the filesystem via cloud-init.

**Default:** not set (overlay matches base image size)

## uefi

```kdl
uefi #true
```

Boot with UEFI firmware (OVMF) instead of legacy BIOS. Required for guests whose
disk image was built with a UEFI-only bootloader (GPT + ESP, no legacy MBR boot
code) — e.g. illumos/OpenIndiana qcow2 images with `bootloader "uefi"`. When
`#true`, vmctl copies a per-VM `OVMF_VARS` alongside the read-only `OVMF_CODE`
pflash; if OVMF firmware isn't found on the host, it falls back to BIOS boot
with a warning.

**Default:** `#false` (legacy BIOS boot)

## virtio-transitional

```kdl
virtio-transitional #true
```

Force `virtio-blk-pci`/`virtio-net-pci` into legacy-transitional mode
(`disable-modern=on,disable-legacy=off`) instead of QEMU's normal
modern-preferred default. Works around a guest-driver bug where the
"modern" virtio-pci capability BAR fails to map under q35/PCIe (QEMU/guest
console shows `Failed to map CAP N @ BARx`, followed by the guest's block
driver failing to attach) — observed with illumos `vioblk`/`vioif` guests
on a q35 machine.

**Default:** `#false` (QEMU's normal default)
