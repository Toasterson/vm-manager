# vmctl reload

Destroy and recreate VMs from VMFile.kdl.

## Synopsis

```
vmctl reload [OPTIONS]
```

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `--file` | path | | Path to VMFile.kdl (auto-discovered if omitted) |
| `--name` | string | | Only reload a specific VM |
| `--no-provision` | flag | `false` | Skip provisioning after reload |

## Details

For each VM: destroys the existing instance (if any), then creates, starts, and provisions a fresh VM from the current VMFile definition. Useful when you've changed the VMFile and want a clean slate.

## Examples

```bash
# Reload all VMs
vmctl reload

# Reload a specific VM
vmctl reload --name webserver

# Reload without provisioning
vmctl reload --no-provision
```

## See Also

[vmctl up](./up.md), [vmctl down](./down.md)
