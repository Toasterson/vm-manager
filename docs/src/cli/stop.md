# vmctl stop

Stop a running VM.

## Synopsis

```
vmctl stop [OPTIONS] <NAME>
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (positional) |

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `--timeout` | integer | `30` | Graceful shutdown timeout in seconds |

## Details

Sends an ACPI power-down signal via QMP. If the guest doesn't shut down within the timeout, vmctl sends SIGTERM to the QEMU process, then SIGKILL as a last resort.

## Examples

```bash
# Stop with default 30-second timeout
vmctl stop myvm

# Give it more time to shut down gracefully
vmctl stop myvm --timeout 120
```

## See Also

[vmctl start](./start.md), [vmctl destroy](./destroy.md)
