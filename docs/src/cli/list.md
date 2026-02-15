# vmctl list

List all registered VMs.

## Synopsis

```
vmctl list
```

## Output

```text
NAME             BACKEND  VCPUS   MEM NETWORK     PID      SSH
webserver        qemu     2       2048 user       12345    10042
database         qemu     4       4096 tap        12346    -
```

| Column | Description |
|---|---|
| `NAME` | VM name |
| `BACKEND` | Hypervisor backend (qemu, propolis, noop) |
| `VCPUS` | Number of virtual CPUs |
| `MEM` | Memory in MB |
| `NETWORK` | Networking mode (user, tap, vnic, none) |
| `PID` | QEMU process PID (or `-` if not running) |
| `SSH` | SSH host port (or `-` if not available) |

## Examples

```bash
vmctl list
```
