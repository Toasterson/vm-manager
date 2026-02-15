# vmctl start

Start an existing VM.

## Synopsis

```
vmctl start <NAME>
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (positional) |

## Details

Starts a VM that is in the `Prepared` or `Stopped` state. The VM must have been previously created with `vmctl create` or `vmctl up`.

## Examples

```bash
vmctl start myvm
```

## See Also

[vmctl stop](./stop.md), [vmctl create](./create.md)
