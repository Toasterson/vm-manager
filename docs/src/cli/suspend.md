# vmctl suspend

Suspend (pause) a running VM.

## Synopsis

```
vmctl suspend <NAME>
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (positional) |

## Details

Pauses the VM's vCPUs via QMP. The VM remains in memory but stops executing. Use `vmctl resume` to continue.

## Examples

```bash
vmctl suspend myvm
```

## See Also

[vmctl resume](./resume.md)
