# vmctl console

Attach to a VM's serial console.

## Synopsis

```
vmctl console <NAME>
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (positional) |

## Details

Connects to the VM's serial console via a Unix socket (QEMU) or WebSocket (Propolis). You'll see the same output as a physical serial port: boot messages, kernel output, and a login prompt.

Press **Ctrl+]** (0x1d) to detach from the console.

## Examples

```bash
vmctl console myvm
```

## See Also

[vmctl ssh](./ssh.md), [vmctl log](./log.md)
