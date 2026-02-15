# vmctl ssh

SSH into a VM.

## Synopsis

```
vmctl ssh [OPTIONS] [NAME]
```

## Arguments

| Argument | Description |
|---|---|
| `NAME` | VM name (optional; inferred from VMFile.kdl if only one VM is defined) |

## Options

| Option | Type | Description |
|---|---|---|
| `--user` | string | SSH username (overrides VMFile) |
| `--key` | path | Path to SSH private key |
| `--file` | path | Path to VMFile.kdl (for reading ssh user) |

## Key Resolution

vmctl searches for a private key in this order:

1. Auto-generated key in VM's work directory (`id_ed25519_generated`)
2. Key specified with `--key`
3. `~/.ssh/id_ed25519`
4. `~/.ssh/id_ecdsa`
5. `~/.ssh/id_rsa`

## User Resolution

1. `--user` CLI flag
2. `user` field in VMFile's `ssh` block
3. Default: `"vm"`

## Details

vmctl first verifies SSH connectivity using libssh2 (with a 30-second retry timeout), then hands off to the system `ssh` binary for full interactive terminal support. SSH options `StrictHostKeyChecking=no` and `UserKnownHostsFile=/dev/null` are set automatically.

For user-mode networking, vmctl connects to `127.0.0.1` on the forwarded host port. For TAP networking, it discovers the guest IP via ARP.

## Examples

```bash
# SSH into the only VM in VMFile.kdl
vmctl ssh

# SSH into a specific VM
vmctl ssh myvm

# Override user and key
vmctl ssh myvm --user root --key ~/.ssh/special_key
```

## See Also

[vmctl console](./console.md)
