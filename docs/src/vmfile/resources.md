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
