# SSH Block

The `ssh` block tells vmctl how to connect to the guest for provisioning and `vmctl ssh`.

## Syntax

```kdl
ssh {
    user "ubuntu"
    private-key "~/.ssh/id_ed25519"
}
```

## Fields

### user

```kdl
user "ubuntu"
```

The SSH username to connect as. This should match the user created by cloud-init.

**Default:** `"vm"` (used when the ssh block exists but `user` is omitted)

### private-key

```kdl
private-key "~/.ssh/id_ed25519"
```

Path to the SSH private key for authentication. Path is resolved relative to the VMFile directory.

**Default:** When omitted, vmctl uses the auto-generated key if available, or falls back to standard keys in `~/.ssh/`.

## When to Include

The `ssh` block is required if you want to:
- Use `vmctl ssh` with VMFile-based name inference.
- Run provisioners (they connect via SSH).

If you only use imperative commands and don't need provisioning, the ssh block is optional.
