# Multi-VM Definitions

A VMFile can define multiple VMs. Each `vm` block is independent.

## Example

```kdl
vm "web" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"
    vcpus 2
    memory 2048

    cloud-init {
        hostname "web"
    }

    ssh {
        user "ubuntu"
    }

    provision "shell" {
        inline "sudo apt-get update && sudo apt-get install -y nginx"
    }
}

vm "db" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"
    vcpus 2
    memory 4096
    disk 50

    cloud-init {
        hostname "db"
    }

    ssh {
        user "ubuntu"
    }

    provision "shell" {
        inline "sudo apt-get update && sudo apt-get install -y postgresql"
    }
}
```

## Behavior with Multi-VM

- `vmctl up` brings up all VMs in order.
- `vmctl down` stops all VMs.
- `vmctl ssh` requires `--name` when multiple VMs are defined (or it will error).
- Use `--name` with any command to target a specific VM.

## Filtering

```bash
vmctl up --name web          # only bring up "web"
vmctl provision --name db    # re-provision only "db"
vmctl down --name web        # stop only "web"
```

## Constraints

- VM names must be unique within the file.
- Each VM is fully independent (no shared networking or cross-references).
