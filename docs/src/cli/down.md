# vmctl down

Bring down VMs defined in VMFile.kdl.

## Synopsis

```
vmctl down [OPTIONS]
```

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `--file` | path | | Path to VMFile.kdl (auto-discovered if omitted) |
| `--name` | string | | Only bring down a specific VM |
| `--destroy` | flag | `false` | Destroy VMs instead of just stopping |

## Details

Without `--destroy`, VMs are stopped gracefully (30-second timeout). They can be restarted with `vmctl up` or `vmctl start`.

With `--destroy`, VMs are fully destroyed: all files removed, unregistered from the store. This is irreversible.

## Examples

```bash
# Stop all VMs in VMFile.kdl
vmctl down

# Stop a specific VM
vmctl down --name webserver

# Destroy all VMs
vmctl down --destroy
```

## See Also

[vmctl up](./up.md), [vmctl destroy](./destroy.md)
