# vmctl destroy

Destroy a VM and clean up all associated resources.

## Synopsis

```
vmctl destroy <NAME>
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (positional) |

## Details

Stops the VM if it's running, then removes all associated files: QCOW2 overlay, cloud-init ISO, log files, SSH keys, sockets, and the work directory. Unregisters the VM from the store.

This action is irreversible.

## Examples

```bash
vmctl destroy myvm
```

## See Also

[vmctl down](./down.md) (declarative equivalent)
