# Cloud-Init Block

The `cloud-init` block configures guest initialization via cloud-init's NoCloud datasource.

## Syntax

```kdl
cloud-init {
    hostname "myvm"
    ssh-key "~/.ssh/id_ed25519.pub"
    user-data "path/to/cloud-config.yaml"
}
```

All fields are optional.

## Fields

### hostname

```kdl
hostname "myvm"
```

Sets the guest hostname via cloud-init metadata.

### ssh-key

```kdl
ssh-key "~/.ssh/id_ed25519.pub"
```

Path to an SSH public key file. The key is injected into the cloud-config's `authorized_keys` for the SSH user. Path is resolved relative to the VMFile directory.

### user-data

```kdl
user-data "cloud-config.yaml"
```

Path to a raw cloud-config YAML file. When this is set, vmctl passes the file contents directly as user-data without generating its own cloud-config. You are responsible for user creation and SSH setup.

**Mutually exclusive with `ssh-key`** in practice - if you provide raw user-data, vmctl won't inject any SSH keys.

## Auto-Generated SSH Keys

When a `cloud-init` block is present but neither `ssh-key` nor `user-data` is specified, vmctl automatically:

1. Generates a per-VM Ed25519 keypair.
2. Injects the public key into the cloud-config.
3. Stores both keys in the VM's work directory.

This is the recommended approach for most use cases.
