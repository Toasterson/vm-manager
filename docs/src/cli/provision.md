# vmctl provision

Re-run provisioners on running VMs from VMFile.kdl.

## Synopsis

```
vmctl provision [OPTIONS]
```

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `--file` | path | | Path to VMFile.kdl (auto-discovered if omitted) |
| `--name` | string | | Only provision a specific VM |

## Details

Re-runs all provision steps defined in the VMFile on already-running VMs. The VM must be running and have an `ssh` block in the VMFile.

vmctl waits up to 120 seconds for SSH to become available, then runs each provisioner in sequence, streaming output to the terminal and logging to `provision.log`.

Useful for iterating on provision scripts without recreating the VM.

## Examples

```bash
# Re-provision all VMs
vmctl provision

# Re-provision a specific VM
vmctl provision --name builder
```

## See Also

[vmctl up](./up.md), [vmctl reload](./reload.md)
