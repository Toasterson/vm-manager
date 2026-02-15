# Cloud-Init and SSH Keys

vmctl uses [cloud-init](https://cloud-init.io/) to configure guests on first boot. It generates a NoCloud seed ISO containing user-data and meta-data, which the guest's cloud-init agent picks up automatically.

## SSH Key Modes

There are three ways to get SSH access to a VM:

### 1. Auto-Generated Keypair (Recommended)

When you define a `cloud-init` block without an explicit `ssh-key`, vmctl generates a per-VM Ed25519 keypair:

```kdl
cloud-init {
    hostname "myvm"
}

ssh {
    user "ubuntu"
}
```

The keys are stored in the VM's work directory:
- `~/.local/share/vmctl/vms/<name>/id_ed25519_generated` (private)
- `~/.local/share/vmctl/vms/<name>/id_ed25519_generated.pub` (public)

This is the simplest option. No key management required.

### 2. Explicit SSH Key

Point to your own public key file:

```kdl
cloud-init {
    ssh-key "~/.ssh/id_ed25519.pub"
}

ssh {
    user "ubuntu"
    private-key "~/.ssh/id_ed25519"
}
```

### 3. Raw User-Data

Provide a complete cloud-config YAML file for full control:

```kdl
cloud-init {
    user-data "./my-cloud-config.yaml"
}
```

In this mode, you're responsible for setting up SSH access yourself in the user-data.

## SSH Key Resolution

When vmctl needs to SSH into a VM (for `vmctl ssh` or provisioning), it searches for a private key in this order:

1. Generated key in the VM's work directory (`id_ed25519_generated`)
2. Key specified with `--key` flag or `private-key` in VMFile ssh block
3. Standard keys in `~/.ssh/`: `id_ed25519`, `id_ecdsa`, `id_rsa`

## SSH User Resolution

1. `--user` CLI flag
2. `user` in VMFile `ssh` block
3. Default: `"vm"`

## Cloud-Init User Setup

When vmctl generates the cloud-config, it creates a user with:
- The specified username
- Passwordless `sudo` access
- The SSH public key in `authorized_keys`
- Bash as the default shell
- Root login disabled
