# Provision Blocks

Provision blocks define steps to run on the guest after boot. They execute in order and abort on the first failure.

## Shell Provisioner

### Inline Command

```kdl
provision "shell" {
    inline "sudo apt-get update && sudo apt-get install -y nginx"
}
```

Executes the command directly on the guest via SSH.

### Script File

```kdl
provision "shell" {
    script "scripts/setup.sh"
}
```

The script file is uploaded to `/tmp/vmctl-provision-<step>.sh` on the guest, made executable with `chmod +x`, and executed. The path is resolved relative to the VMFile directory.

### Validation

A shell provisioner must have exactly one of `inline` or `script`. Specifying both or neither is an error.

## File Provisioner

```kdl
provision "file" {
    source "config/app.conf"
    destination "/etc/app/app.conf"
}
```

Uploads a local file to the guest via SFTP.

### Required Fields

| Field | Description |
|---|---|
| `source` | Local file path (relative to VMFile directory) |
| `destination` | Absolute path on the guest |

## Execution Behavior

- Provisioners run sequentially in the order they appear.
- Shell provisioners stream stdout and stderr to your terminal in real-time.
- A non-zero exit code from any shell provisioner aborts the sequence.
- All output is also logged to `provision.log` in the VM's work directory.
- vmctl waits up to 120 seconds for SSH to become available before starting provisioners.
