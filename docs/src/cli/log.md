# vmctl log

Show VM console and provision logs.

## Synopsis

```
vmctl log [OPTIONS] <NAME>
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (positional) |

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `--console` | flag | `false` | Show only console log (boot / cloud-init output) |
| `--provision` | flag | `false` | Show only provision log |
| `--tail`, `-n` | integer | `0` | Show the last N lines (0 = all) |

## Details

By default (no flags), both console and provision logs are shown. The console log captures serial output (boot messages, cloud-init output). The provision log captures stdout/stderr from provisioner runs.

Log files are located in the VM's work directory:
- `console.log` - Serial console output
- `provision.log` - Provisioning output

## Examples

```bash
# Show all logs
vmctl log myvm

# Show only provision output
vmctl log myvm --provision

# Show last 50 lines of console log
vmctl log myvm --console --tail 50
```

## See Also

[vmctl console](./console.md), [vmctl status](./status.md)
