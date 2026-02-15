# Introduction

**vmctl** is a command-line tool for creating, managing, and provisioning virtual machines on Linux (QEMU/KVM) and illumos (Propolis/bhyve). It offers both imperative commands for one-off tasks and a declarative configuration format (`VMFile.kdl`) for reproducible VM environments.

## Why vmctl?

Managing VMs with raw QEMU commands is tedious and error-prone. vmctl handles the plumbing: disk overlays, cloud-init ISOs, SSH key generation, network configuration, and process lifecycle. You describe *what* you want; vmctl figures out *how*.

Think of it like this:

| Docker world | vmctl world |
|---|---|
| `docker run` | `vmctl create --start` |
| `docker-compose.yml` | `VMFile.kdl` |
| `docker compose up` | `vmctl up` |
| `docker compose down` | `vmctl down` |

## A Taste

Create a `VMFile.kdl`:

```kdl
vm "dev" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"
    vcpus 2
    memory 2048

    cloud-init {
        hostname "dev"
    }

    ssh {
        user "ubuntu"
    }

    provision "shell" {
        inline "sudo apt-get update && sudo apt-get install -y build-essential"
    }
}
```

Then:

```bash
vmctl up      # download image, create VM, boot, provision
vmctl ssh     # connect over SSH
vmctl down    # shut it down
```

## Platform Support

| Platform | Backend | Status |
|---|---|---|
| Linux | QEMU/KVM | Fully supported |
| illumos | Propolis/bhyve | Experimental |

## Project Structure

vmctl is split into two crates:

- **vm-manager** - Library crate with the hypervisor abstraction, image management, SSH, provisioning, and VMFile parsing.
- **vmctl** - CLI binary built on top of vm-manager.

Both live in a Cargo workspace under `crates/`.
