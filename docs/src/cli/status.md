# vmctl status

Show detailed status of a VM.

## Synopsis

```
vmctl status <NAME>
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (positional) |

## Output

Displays all known information about the VM:

- Name, ID, Backend, State
- vCPUs, Memory, Disk
- Network configuration (mode, bridge name)
- Work directory path
- Overlay path, Seed ISO path
- PID, VNC address
- SSH port, MAC address

## Examples

```bash
vmctl status myvm
```

## See Also

[vmctl list](./list.md)
