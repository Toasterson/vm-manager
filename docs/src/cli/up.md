# vmctl up

Bring up VMs defined in VMFile.kdl.

## Synopsis

```
vmctl up [OPTIONS]
```

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `--file` | path | | Path to VMFile.kdl (auto-discovered if omitted) |
| `--name` | string | | Only bring up a specific VM |
| `--no-provision` | flag | `false` | Skip provisioning steps |

## Details

For each VM in the VMFile:

1. If the VM is **already running**, it is skipped.
2. If the VM exists but is **stopped**, it is restarted and re-provisioned.
3. If the VM **doesn't exist**, it is created, started, and provisioned.

Images are downloaded and cached as needed. SSH keys are auto-generated when cloud-init is configured without an explicit key.

## Examples

```bash
# Bring up all VMs in ./VMFile.kdl
vmctl up

# Bring up a specific VM
vmctl up --name webserver

# Bring up without provisioning
vmctl up --no-provision

# Use a specific VMFile
vmctl up --file path/to/VMFile.kdl
```

## See Also

[vmctl down](./down.md), [vmctl reload](./reload.md)
