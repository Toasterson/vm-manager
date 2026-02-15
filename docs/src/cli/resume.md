# vmctl resume

Resume a suspended VM.

## Synopsis

```
vmctl resume <NAME>
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (positional) |

## Details

Resumes a VM that was paused with `vmctl suspend`. The VM continues from exactly where it left off.

## Examples

```bash
vmctl resume myvm
```

## See Also

[vmctl suspend](./suspend.md)
