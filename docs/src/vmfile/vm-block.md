# VM Block

The `vm` block is the top-level element in a VMFile. It defines a single virtual machine.

## Syntax

```kdl
vm "name" {
    // configuration nodes
}
```

The name is a required string argument. It must be unique across all `vm` blocks in the file.

## Example

```kdl
vm "dev-server" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"
    vcpus 2
    memory 2048
    disk 20

    cloud-init {
        hostname "dev-server"
    }

    ssh {
        user "ubuntu"
    }
}
```

## Name Requirements

- Must be a non-empty string.
- Must be unique within the VMFile.
- Used as the VM identifier in `vmctl list`, `vmctl ssh`, `--name` filtering, etc.
- Used as the work directory name under `~/.local/share/vmctl/vms/`.
